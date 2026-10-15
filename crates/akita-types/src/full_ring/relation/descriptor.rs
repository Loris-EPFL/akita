//! Canonical binding of the implicit coefficient-row compiler. Matrix entries
//! are intentionally not serialized: the surrounding instance must bind their
//! original setup key/role, and native_context derives their exact views from
//! that setup. This descriptor binds which role, shape and columns are used.

use super::*;
use jolt_field::{CanonicalBytes, CanonicalEncoding};

const DESCRIPTOR_VERSION: &[u8] = b"akita/full-ring/coefficient-relation/v1";

impl<K: Field + CanonicalEncoding, E: ExtField<K>> CoefficientRelationPlan<'_, K, E> {
    /// Canonical complete strong-relation binding, to be absorbed after the
    /// outgoing source/A images and BEFORE tau0 and rho. Integer sizes, indices
    /// and vector lengths are LE64. K values use their fixed canonical LE byte
    /// encoding; E values use increasing canonical K-coordinate order.
    ///
    /// Row tags are A=0, L=1, B=2, D=3, F=4, H=5, scalar=6. Matrix role tags
    /// are A=0, B=1, D=2, F=3, H=4, with explicit group/stage indices. All
    /// actual dimensions, digit maps, source permutations, input spans, query
    /// and challenge coefficients, scalar rows and support intervals are bound.
    /// The surrounding instance's setup/key binding is REQUIRED: this is not
    /// a standalone commitment to arbitrary caller-supplied matrix entries.
    /// Every matrix must therefore originate from NativeMatrix::from_installed;
    /// an unregistered reference matrix has no admissible descriptor.
    pub fn canonical_descriptor_bytes(&self) -> Result<Vec<u8>, AkitaError> {
        let mut out = DescriptorWriter { bytes: Vec::new() };
        out.bytes.extend_from_slice(DESCRIPTOR_VERSION);
        out.bytes
            .extend_from_slice(&akita_params::field_modulus_be_bytes::<K>()?);
        out.usize(K::NUM_BYTES)?;
        out.usize(E::DEGREE)?;
        // Bind the actual coordinate algebra, not just an extension degree.
        // This is 8 K values for the selected quadratic extension.
        for left in 0..E::DEGREE {
            let a = E::from_base_fn(|coordinate| {
                if coordinate == left {
                    K::one()
                } else {
                    K::zero()
                }
            });
            for right in 0..E::DEGREE {
                let b = E::from_base_fn(|coordinate| {
                    if coordinate == right {
                        K::one()
                    } else {
                        K::zero()
                    }
                });
                out.extension::<K, E>(a * b)?;
            }
        }
        out.usize(self.domain_len)?;
        out.usize(self.rows)?;
        out.usize(self.padded_rows)?;
        out.usize(self.groups.len())?;
        for (index, group) in self.groups.iter().enumerate() {
            out.usize(index)?;
            out.source_view(group.view)?;
            // This original map determines both L's sigma column adjoint and
            // its c[beta(raw_block)] pairing; no copied fold-map descriptor.
            out.matrix(0, index, 0, &group.a)?;
            out.matrix(1, index, 0, &group.b)?;
            out.base_slice(group.challenges)?;
            out.digits(&group.response)?;
            out.digits(&group.inner_images)?;
            out.usize(group.channels.len())?;
            for (channel_index, channel) in group.channels.iter().enumerate() {
                out.usize(channel_index)?;
                // The checked point contains exactly this group's view; its
                // axis weights are determined by these LSB-first coordinates.
                out.extension_slice::<K, E>(channel.point.point())?;
                out.digits(&channel.partials)?;
                out.usize(channel.scalar_claims.len())?;
                for claim in &channel.scalar_claims {
                    out.extension_slice::<K, E>(&claim.weights)?;
                    out.extension::<K, E>(claim.value)?;
                }
            }
            out.chain(3, index, &group.f)?;
        }
        out.matrix(2, 0, 0, &self.d)?;
        out.chain(4, 0, &self.h)?;
        out.usize(self.manifest.len())?;
        for block in &self.manifest {
            match block.family {
                CoefficientRowFamily::A { group } => {
                    out.tag(0);
                    out.usize(group)?;
                }
                CoefficientRowFamily::L {
                    group,
                    channel,
                    plane,
                } => {
                    out.tag(1);
                    out.usize(group)?;
                    out.usize(channel)?;
                    out.usize(plane)?;
                }
                CoefficientRowFamily::B { group } => {
                    out.tag(2);
                    out.usize(group)?;
                }
                CoefficientRowFamily::D => out.tag(3),
                CoefficientRowFamily::F { group, stage } => {
                    out.tag(4);
                    out.usize(group)?;
                    out.usize(stage)?;
                }
                CoefficientRowFamily::H { stage } => {
                    out.tag(5);
                    out.usize(stage)?;
                }
                CoefficientRowFamily::Scalar {
                    group,
                    channel,
                    claim,
                } => {
                    out.tag(6);
                    out.usize(group)?;
                    out.usize(channel)?;
                    out.usize(claim)?;
                }
            }
            out.range(&block.rows)?;
        }
        out.support(&self.binary)?;
        out.support(&self.zeros)?;
        Ok(out.bytes)
    }
}

struct DescriptorWriter {
    bytes: Vec<u8>,
}
impl DescriptorWriter {
    fn tag(&mut self, tag: u8) {
        self.bytes.push(tag);
    }
    fn usize(&mut self, value: usize) -> Result<(), AkitaError> {
        let value = u64::try_from(value)
            .map_err(|_| invalid("coefficient descriptor integer exceeds LE64"))?;
        self.bytes.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }
    fn range(&mut self, range: &Range<usize>) -> Result<(), AkitaError> {
        self.usize(range.start)?;
        self.usize(range.end)
    }
    fn base<K: CanonicalBytes>(&mut self, value: K) -> Result<(), AkitaError> {
        let start = self.bytes.len();
        let end = start
            .checked_add(K::NUM_BYTES)
            .ok_or_else(|| invalid("coefficient descriptor size overflow"))?;
        self.bytes
            .try_reserve(K::NUM_BYTES)
            .map_err(|_| invalid("coefficient descriptor allocation failed"))?;
        self.bytes.resize(end, 0);
        value.to_bytes_le(&mut self.bytes[start..end]);
        Ok(())
    }
    fn base_slice<K: CanonicalBytes + Copy>(&mut self, values: &[K]) -> Result<(), AkitaError> {
        self.usize(values.len())?;
        for &value in values {
            self.base(value)?;
        }
        Ok(())
    }
    fn extension<K: Field + CanonicalBytes, E: ExtField<K>>(
        &mut self,
        value: E,
    ) -> Result<(), AkitaError> {
        for coordinate in 0..E::DEGREE {
            self.base(value.base_coefficient(coordinate))?;
        }
        Ok(())
    }
    fn extension_slice<K: Field + CanonicalBytes, E: ExtField<K>>(
        &mut self,
        values: &[E],
    ) -> Result<(), AkitaError> {
        self.usize(values.len())?;
        for &value in values {
            self.extension::<K, E>(value)?;
        }
        Ok(())
    }
    fn permutation(&mut self, permutation: Option<&[usize]>) -> Result<(), AkitaError> {
        match permutation {
            None => self.tag(0),
            Some(permutation) => {
                self.tag(1);
                self.usize(permutation.len())?;
                for &index in permutation {
                    self.usize(index)?;
                }
            }
        }
        Ok(())
    }
    fn source_view(&mut self, view: &RawSourceViewPlan) -> Result<(), AkitaError> {
        // RawCoefficientV1: low-to-high (j,ell,x,i,p), identity K encoding.
        self.tag(1);
        for dimension in [
            view.degree(),
            view.polynomials(),
            view.blocks(),
            view.positions(),
            view.digits(),
            view.padded_polynomials(),
            view.padded_digits(),
            view.physical_len(),
            view.raw_len(),
        ] {
            self.usize(dimension)?;
        }
        self.permutation(view.permutation())
    }
    fn digits<K: Field + CanonicalBytes>(
        &mut self,
        digits: &DigitLayout<K>,
    ) -> Result<(), AkitaError> {
        // BlockDigitCoefficientV1, followed by any registered storage map.
        self.tag(1);
        self.range(&digits.range())?;
        for dimension in [digits.elements, digits.degree, digits.depth] {
            self.usize(dimension)?;
        }
        self.base_slice(&digits.powers)?;
        self.permutation(digits.permutation.as_deref())
    }
    fn matrix<K>(
        &mut self,
        role: u8,
        group: usize,
        stage: usize,
        matrix: &NativeMatrix<'_, K>,
    ) -> Result<(), AkitaError> {
        self.tag(role);
        self.usize(group)?;
        self.usize(stage)?;
        for dimension in [matrix.rows, matrix.columns, matrix.degree] {
            self.usize(dimension)?;
        }
        match matrix.installed_role {
            None => {
                return Err(invalid(
                    "canonical relation binding requires original installed matrix roles",
                ))
            }
            Some(role) => {
                self.tag(1);
                use crate::FullRingMatrixRole;
                match role {
                    FullRingMatrixRole::A { source } => {
                        self.tag(0);
                        self.bytes.extend_from_slice(&source.0.to_le_bytes());
                    }
                    FullRingMatrixRole::B { source } => {
                        self.tag(1);
                        self.bytes.extend_from_slice(&source.0.to_le_bytes());
                    }
                    FullRingMatrixRole::F { source, stage } => {
                        self.tag(2);
                        self.bytes.extend_from_slice(&source.0.to_le_bytes());
                        self.tag(stage);
                    }
                    FullRingMatrixRole::D => self.tag(3),
                    FullRingMatrixRole::H { stage } => {
                        self.tag(4);
                        self.tag(stage);
                    }
                    FullRingMatrixRole::TerminalA => self.tag(5),
                    FullRingMatrixRole::Registered { index } => {
                        self.tag(6);
                        self.bytes.extend_from_slice(&index.to_le_bytes());
                    }
                }
            }
        }
        Ok(())
    }
    fn chain<K: Field + CanonicalBytes>(
        &mut self,
        role: u8,
        group: usize,
        chain: &CompressionChain<'_, K>,
    ) -> Result<(), AkitaError> {
        self.usize(chain.stages.len())?;
        for (stage, map) in chain.stages.iter().enumerate() {
            self.matrix(role, group, stage, &map.matrix)?;
            self.range(&map.input)?;
            self.usize(map.digit_bits)?;
            // NegativeBinaryTransposeV1: coefficient v, digit u at v+s*u.
            self.tag(1);
        }
        self.base_slice(chain.payload)
    }
    fn support(&mut self, support: &CoefficientSupport) -> Result<(), AkitaError> {
        self.usize(support.domain_len())?;
        self.usize(support.intervals().len())?;
        for span in support.intervals() {
            self.range(span)?;
        }
        Ok(())
    }
}

impl crate::full_ring::FinalReceiverDomainPlan {
    /// Bind the original identity source geometry and exact predecessor
    /// binary/zero supports before the final receiver domain's tau0 draw.
    /// Source commitment/key and the pending ordinary claim remain part of
    /// the surrounding owner-bound transcript context.
    pub fn canonical_descriptor_bytes(&self) -> Result<Vec<u8>, AkitaError> {
        let mut out = DescriptorWriter { bytes: Vec::new() };
        out.bytes
            .extend_from_slice(b"akita/full-ring/final-receiver-domain/v1");
        out.source_view(self.view())?;
        out.support(self.binary_support())?;
        out.support(self.required_zero_support())?;
        Ok(out.bytes)
    }
}
