//! Exact quotient-free coefficient rows for the complete native successor.
//!
//! The anchor is the original A/B/D/F/H maps and every supplied path digit.
//! Ring products reduce with their native signed wrap, never an off-ring alpha
//! evaluation. One visitor defines column accumulation, point MLEs and direct
//! row evaluation, so the prover and verifier cannot select different maps.

use super::{invalid, product, CoefficientSupport, FullRingRawPoint, RawSourceViewPlan};
use akita_algebra::{eq_poly::EqPolynomial, offset_eq::OffsetEqWindow};
use akita_error::AkitaError;
use jolt_field::{ExtField, Field};
use std::{ops::Range, sync::Arc};

mod descriptor;

/// Borrowed native matrix in [row][column][coefficient] order.
#[derive(Clone, Copy, Debug)]
pub struct NativeMatrix<'a, K> {
    rows: usize,
    columns: usize,
    degree: usize,
    coefficients: &'a [K],
    installed_role: Option<crate::FullRingMatrixRole>,
}
impl<'a, K> NativeMatrix<'a, K> {
    pub fn new(
        rows: usize,
        columns: usize,
        degree: usize,
        coefficients: &'a [K],
    ) -> Result<Self, AkitaError> {
        if rows == 0
            || columns == 0
            || !degree.is_power_of_two()
            || coefficients.len() != product(&[rows, columns, degree])?
        {
            return Err(invalid("native coefficient matrix geometry"));
        }
        Ok(Self {
            rows,
            columns,
            degree,
            coefficients,
            installed_role: None,
        })
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn columns(&self) -> usize {
        self.columns
    }
    pub fn degree(&self) -> usize {
        self.degree
    }
    pub fn coefficients(&self) -> &'a [K] {
        self.coefficients
    }
    pub fn installed_role(&self) -> Option<crate::FullRingMatrixRole> {
        self.installed_role
    }
    fn coefficient(&self, row: usize, column: usize, index: usize) -> K
    where
        K: Copy,
    {
        self.coefficients[(row * self.columns + column) * self.degree + index]
    }
    fn output_len(&self) -> usize {
        self.rows * self.degree
    }
    fn input_len(&self) -> usize {
        self.columns * self.degree
    }
}

impl<'a, K: Field> NativeMatrix<'a, K> {
    /// Preserve the exact original installed role alongside its borrowed
    /// coefficients so canonical row binding cannot interchange equal shapes.
    pub fn from_installed(
        setup: &'a crate::InstalledFullRingSetup<K>,
        role: crate::FullRingMatrixRole,
    ) -> Result<Self, AkitaError> {
        let shape = setup.descriptor().matrix(role)?;
        let coefficients = setup.matrix_coefficients(role)?;
        let mut matrix = Self::new(
            shape.rows,
            shape.columns,
            shape.ring_dimension,
            coefficients,
        )?;
        matrix.installed_role = Some(role);
        Ok(matrix)
    }
}

/// Native digit layout [element][digit][coefficient]. A physical permutation
/// can express other registered layouts without constructing new witness data.
#[derive(Clone, Debug)]
pub struct DigitLayout<K> {
    start: usize,
    elements: usize,
    degree: usize,
    depth: usize,
    powers: Vec<K>,
    len: usize,
    permutation: Option<Arc<[usize]>>,
}
impl<K: Field> DigitLayout<K> {
    pub fn new(
        start: usize,
        elements: usize,
        degree: usize,
        depth: usize,
        base: K,
    ) -> Result<Self, AkitaError> {
        if elements == 0 || depth == 0 || !degree.is_power_of_two() {
            return Err(invalid("digit layout geometry"));
        }
        let len = product(&[elements, degree, depth])?;
        start
            .checked_add(len)
            .ok_or_else(|| invalid("digit layout extent overflow"))?;
        let mut powers = Vec::with_capacity(depth);
        let mut power = K::one();
        for _ in 0..depth {
            powers.push(power);
            power *= base;
        }
        Ok(Self {
            start,
            elements,
            degree,
            depth,
            powers,
            len,
            permutation: None,
        })
    }
    pub fn with_permutation(mut self, permutation: Vec<usize>) -> Result<Self, AkitaError> {
        if permutation.len() != self.len {
            return Err(invalid("digit permutation length"));
        }
        let mut seen = vec![false; self.len];
        for &index in &permutation {
            let slot = seen
                .get_mut(index)
                .ok_or_else(|| invalid("digit permutation index"))?;
            if *slot {
                return Err(invalid("digit permutation is not injective"));
            }
            *slot = true;
        }
        self.permutation = Some(permutation.into());
        Ok(self)
    }
    pub fn range(&self) -> Range<usize> {
        self.start..self.start + self.len
    }
    pub fn elements(&self) -> usize {
        self.elements
    }
    pub fn degree(&self) -> usize {
        self.degree
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn powers(&self) -> &[K] {
        &self.powers
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn column(
        &self,
        element: usize,
        digit: usize,
        coefficient: usize,
    ) -> Result<usize, AkitaError> {
        if element >= self.elements || digit >= self.depth || coefficient >= self.degree {
            return Err(invalid("digit layout coordinate"));
        }
        Ok(self.address(element, digit, coefficient))
    }
    fn address(&self, element: usize, digit: usize, coefficient: usize) -> usize {
        let offset = (element * self.depth + digit) * self.degree + coefficient;
        self.start + self.permutation.as_ref().map_or(offset, |map| map[offset])
    }
    // Native B/D act on the actual canonical digit vector, before any optional
    // storage permutation. This helper is its physical witness address.
    fn flat_address(&self, offset: usize) -> usize {
        self.start + self.permutation.as_ref().map_or(offset, |map| map[offset])
    }
}

/// A retained compression input, already in its native ring packing. The
/// preceding image coefficient v has digit u at v + image_len*u. This is not
/// the coefficient-major decomposition used for E/T.
#[derive(Clone, Debug)]
pub struct CompressionStage<'a, K> {
    pub matrix: NativeMatrix<'a, K>,
    pub input: Range<usize>,
    pub digit_bits: usize,
}
#[derive(Clone, Debug)]
pub struct CompressionChain<'a, K> {
    pub stages: Vec<CompressionStage<'a, K>>,
    pub payload: &'a [K],
}

/// A scalar functional of complete authenticated partials. Tail weights are
/// in RAW [p][block][coefficient] order, even when the registered beta map
/// relabels physical blocks. Each scalar is kept as DEGREE separate K rows.
/// Distinct original claims are never batched for free.
#[derive(Clone, Debug)]
pub struct PartialScalarClaim<E> {
    pub weights: Vec<E>,
    pub value: E,
}
impl<E: Field> PartialScalarClaim<E> {
    pub fn raw(point: &FullRingRawPoint<E>, value: E) -> Self {
        let view = point.view();
        let mut weights = Vec::with_capacity(view.polynomials() * view.blocks() * view.degree());
        for p in 0..view.polynomials() {
            for i in 0..view.blocks() {
                let block = point.polynomial_weights()[p] * point.block_weights()[i];
                weights.extend(
                    point
                        .coefficient_weights()
                        .iter()
                        .map(|&coefficient| block * coefficient),
                );
            }
        }
        Self { weights, value }
    }
}

#[derive(Clone, Debug)]
pub struct FullRingRelationChannel<'a, K, E> {
    pub point: &'a FullRingRawPoint<E>,
    /// [p][block][plane] elements, each in block-digit-coefficient order.
    pub partials: DigitLayout<K>,
    pub scalar_claims: Vec<PartialScalarClaim<E>>,
}
#[derive(Clone, Debug)]
pub struct FullRingRelationGroup<'a, K, E> {
    pub view: &'a RawSourceViewPlan,
    pub a: NativeMatrix<'a, K>,
    pub b: NativeMatrix<'a, K>,
    /// The original full S challenge for every [p][block], flattened in that
    /// order. The SAME challenge acts independently on each carrier plane.
    pub challenges: &'a [K],
    pub response: DigitLayout<K>,
    pub inner_images: DigitLayout<K>,
    pub channels: Vec<FullRingRelationChannel<'a, K, E>>,
    pub f: CompressionChain<'a, K>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoefficientRowFamily {
    A {
        group: usize,
    },
    L {
        group: usize,
        channel: usize,
        plane: usize,
    },
    B {
        group: usize,
    },
    D,
    F {
        group: usize,
        stage: usize,
    },
    H {
        stage: usize,
    },
    Scalar {
        group: usize,
        channel: usize,
        claim: usize,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoefficientRowBlock {
    pub family: CoefficientRowFamily,
    pub rows: Range<usize>,
}

/// Complete checked implicit M W = b. Public matrices and original payloads
/// are borrowed; storage is O(groups + channels + rows), never O(R*N).
/// Construction validates geometry and coverage, not transcript provenance or
/// setup admission: callers must obtain these inputs from bound native handles.
pub struct CoefficientRelationPlan<'a, K, E> {
    domain_len: usize,
    groups: Vec<FullRingRelationGroup<'a, K, E>>,
    d: NativeMatrix<'a, K>,
    h: CompressionChain<'a, K>,
    manifest: Vec<CoefficientRowBlock>,
    rows: usize,
    padded_rows: usize,
    binary: CoefficientSupport,
    zeros: CoefficientSupport,
}

impl<'a, K: Field, E: ExtField<K>> CoefficientRelationPlan<'a, K, E> {
    pub fn new(
        domain_len: usize,
        groups: Vec<FullRingRelationGroup<'a, K, E>>,
        d: NativeMatrix<'a, K>,
        h: CompressionChain<'a, K>,
    ) -> Result<Self, AkitaError> {
        if !domain_len.is_power_of_two() || groups.is_empty() || E::DEGREE == 0 {
            return Err(invalid("coefficient relation domain"));
        }
        let mut used = Vec::new();
        let mut binary = Vec::new();
        let mut manifest = Vec::new();
        let mut rows = 0usize;
        let mut push_rows = |family, count: usize| -> Result<(), AkitaError> {
            let end = rows
                .checked_add(count)
                .ok_or_else(|| invalid("coefficient row count overflow"))?;
            manifest.push(CoefficientRowBlock {
                family,
                rows: rows..end,
            });
            rows = end;
            Ok(())
        };
        let mut d_input_len = 0usize;
        for (g, group) in groups.iter().enumerate() {
            let view = group.view;
            view.validate_fold_admission()?;
            let blocks = product(&[view.polynomials(), view.blocks()])?;
            let degree = view.degree();
            if group.a.degree != degree
                || group.a.columns != product(&[view.positions(), view.digits()])?
                || group.challenges.len() != product(&[blocks, degree])?
                || group.response.elements != group.a.columns
                || group.response.degree != degree
                || group.inner_images.elements != product(&[blocks, group.a.rows])?
                || group.inner_images.degree != degree
                || group.b.input_len() != group.inner_images.len
                || group.channels.is_empty()
            {
                return Err(invalid("full-ring relation group geometry"));
            }
            used.push(group.response.range());
            used.push(group.inner_images.range());
            push_rows(CoefficientRowFamily::A { group: g }, group.a.output_len())?;
            for (channel_index, channel) in group.channels.iter().enumerate() {
                if channel.point.view() != view
                    || channel.partials.degree != degree
                    || channel.partials.elements != product(&[blocks, E::DEGREE])?
                    || channel.scalar_claims.is_empty()
                    || channel
                        .scalar_claims
                        .iter()
                        .any(|claim| claim.weights.len() != blocks * degree)
                {
                    return Err(invalid("full-ring channel geometry or scalar claims"));
                }
                used.push(channel.partials.range());
                d_input_len = d_input_len
                    .checked_add(channel.partials.len)
                    .ok_or_else(|| invalid("D input overflow"))?;
                for plane in 0..E::DEGREE {
                    push_rows(
                        CoefficientRowFamily::L {
                            group: g,
                            channel: channel_index,
                            plane,
                        },
                        degree,
                    )?;
                }
            }
            push_rows(CoefficientRowFamily::B { group: g }, group.b.output_len())?;
            validate_chain(
                &group.f,
                group.b.output_len(),
                domain_len,
                &mut used,
                &mut binary,
            )?;
        }
        if d.input_len() != d_input_len {
            return Err(invalid("shared D omits or duplicates partial planes"));
        }
        validate_chain(&h, d.output_len(), domain_len, &mut used, &mut binary)?;
        push_rows(CoefficientRowFamily::D, d.output_len())?;
        // Complete stage-major tail/row ordering, all original F followed by H.
        let stage_count = groups
            .iter()
            .map(|g| g.f.stages.len())
            .chain(std::iter::once(h.stages.len()))
            .max()
            .unwrap_or(0);
        for stage in 0..stage_count {
            for (g, group) in groups.iter().enumerate() {
                if let Some(map) = group.f.stages.get(stage) {
                    push_rows(
                        CoefficientRowFamily::F { group: g, stage },
                        map.matrix.output_len(),
                    )?;
                }
            }
            if let Some(map) = h.stages.get(stage) {
                push_rows(CoefficientRowFamily::H { stage }, map.matrix.output_len())?;
            }
        }
        for (g, group) in groups.iter().enumerate() {
            for (channel, opening) in group.channels.iter().enumerate() {
                for claim in 0..opening.scalar_claims.len() {
                    push_rows(
                        CoefficientRowFamily::Scalar {
                            group: g,
                            channel,
                            claim,
                        },
                        E::DEGREE,
                    )?;
                }
            }
        }
        used.sort_unstable_by_key(|span| span.start);
        binary.sort_unstable_by_key(|span| span.start);
        let mut end = 0;
        let mut zero_spans = Vec::new();
        for span in &used {
            if span.start < end || span.start >= span.end || span.end > domain_len {
                return Err(invalid(
                    "overlapping or out-of-domain complete witness spans",
                ));
            }
            if span.start > end {
                zero_spans.push(end..span.start);
            }
            end = span.end;
        }
        if end < domain_len {
            zero_spans.push(end..domain_len);
        }
        let padded_rows = rows
            .checked_next_power_of_two()
            .ok_or_else(|| invalid("coefficient row padding overflow"))?;
        Ok(Self {
            domain_len,
            groups,
            d,
            h,
            manifest,
            rows,
            padded_rows,
            binary: CoefficientSupport::new(domain_len, binary)?,
            zeros: CoefficientSupport::new(domain_len, zero_spans)?,
        })
    }

    /// Selected-family entry: additionally checks q, the actual E basis and the
    /// full S128 carrier. Geometry remains per-instance (inner versus outer).
    pub fn new_d1(
        domain_len: usize,
        groups: Vec<FullRingRelationGroup<'a, K, E>>,
        d: NativeMatrix<'a, K>,
        h: CompressionChain<'a, K>,
    ) -> Result<Self, AkitaError>
    where
        K: jolt_field::CanonicalEncoding,
    {
        super::validate_full_ring_fields::<K, E>()?;
        if groups.iter().any(|group| group.view.degree() != 128) {
            return Err(invalid("D1 carrier must be full S128"));
        }
        Self::new(domain_len, groups, d, h)
    }
    pub fn domain_len(&self) -> usize {
        self.domain_len
    }
    pub fn num_vars(&self) -> usize {
        self.domain_len.trailing_zeros() as usize
    }
    pub fn row_count(&self) -> usize {
        self.rows
    }
    pub fn padded_row_count(&self) -> usize {
        self.padded_rows
    }
    pub fn row_num_vars(&self) -> usize {
        self.padded_rows.trailing_zeros() as usize
    }
    pub fn row_manifest(&self) -> &[CoefficientRowBlock] {
        &self.manifest
    }
    pub fn binary_support(&self) -> &CoefficientSupport {
        &self.binary
    }
    pub fn required_zero_support(&self) -> &CoefficientSupport {
        &self.zeros
    }

    /// Stream exact K entries and RHS entries of the same implicit matrix.
    /// Duplicate (row,column) contributions are deliberately additive.
    pub fn visit_entries(
        &self,
        mut entry: impl FnMut(usize, usize, K),
        mut rhs: impl FnMut(usize, K),
    ) {
        for block in &self.manifest {
            let start = block.rows.start;
            match block.family {
                CoefficientRowFamily::A { group: g } => {
                    let group = &self.groups[g];
                    let degree = group.a.degree;
                    visit_native(&group.a, start, |flat, weight, row| {
                        let element = flat / degree;
                        let j = flat % degree;
                        for (digit, &power) in group.response.powers.iter().enumerate() {
                            entry(
                                row,
                                group.response.address(element, digit, j),
                                weight * power,
                            );
                        }
                    });
                    let blocks = group.view.polynomials() * group.view.blocks();
                    for image in 0..group.a.rows {
                        for b in 0..blocks {
                            let challenge = &group.challenges[b * degree..(b + 1) * degree];
                            visit_multiplier(challenge, |t, j, weight| {
                                for (digit, &power) in group.inner_images.powers.iter().enumerate()
                                {
                                    entry(
                                        start + image * degree + t,
                                        group.inner_images.address(
                                            b * group.a.rows + image,
                                            digit,
                                            j,
                                        ),
                                        -weight * power,
                                    );
                                }
                            });
                        }
                    }
                }
                CoefficientRowFamily::L {
                    group: g,
                    channel: c,
                    plane,
                } => {
                    let group = &self.groups[g];
                    let channel = &group.channels[c];
                    let degree = group.view.degree();
                    for x in 0..group.view.positions() {
                        for ell in 0..group.view.digits() {
                            let physical_column =
                                group.view.fold_column(x * group.view.digits() + ell);
                            let weight = (channel.point.position_weights()[x]
                                * channel.point.digit_weights()[ell])
                                .base_coefficient(plane);
                            for (digit, &power) in group.response.powers.iter().enumerate() {
                                for j in 0..degree {
                                    entry(
                                        start + j,
                                        group.response.address(physical_column, digit, j),
                                        weight * power,
                                    );
                                }
                            }
                        }
                    }
                    for b in 0..group.view.polynomials() * group.view.blocks() {
                        // Raw partial b came from physical block beta(b).
                        // A and the source fold retain physical shell order.
                        let physical_block = group.view.fold_block(b);
                        visit_multiplier(
                            &group.challenges
                                [physical_block * degree..(physical_block + 1) * degree],
                            |t, j, weight| {
                                for (digit, &power) in channel.partials.powers.iter().enumerate() {
                                    entry(
                                        start + t,
                                        channel.partials.address(b * E::DEGREE + plane, digit, j),
                                        -weight * power,
                                    );
                                }
                            },
                        );
                    }
                }
                CoefficientRowFamily::B { group: g } => {
                    let group = &self.groups[g];
                    visit_native(&group.b, start, |flat, weight, row| {
                        entry(row, group.inner_images.flat_address(flat), weight)
                    });
                    visit_image_target(
                        &group.f,
                        0,
                        group.b.output_len(),
                        start,
                        &mut entry,
                        &mut rhs,
                    );
                }
                CoefficientRowFamily::D => {
                    // Matrix columns follow complete ordered group/channel E
                    // segments, each including every original block and plane.
                    let mut offset = 0;
                    for group in &self.groups {
                        for channel in &group.channels {
                            let end = offset + channel.partials.len;
                            visit_native_interval(
                                &self.d,
                                start,
                                offset..end,
                                |flat, weight, row| {
                                    entry(
                                        row,
                                        channel.partials.flat_address(flat - offset),
                                        weight,
                                    );
                                },
                            );
                            offset = end;
                        }
                    }
                    visit_image_target(
                        &self.h,
                        0,
                        self.d.output_len(),
                        start,
                        &mut entry,
                        &mut rhs,
                    );
                }
                CoefficientRowFamily::F { group, stage } => {
                    visit_compression_stage(
                        &self.groups[group].f,
                        stage,
                        start,
                        &mut entry,
                        &mut rhs,
                    );
                }
                CoefficientRowFamily::H { stage } => {
                    visit_compression_stage(&self.h, stage, start, &mut entry, &mut rhs);
                }
                CoefficientRowFamily::Scalar {
                    group: g,
                    channel: c,
                    claim: a,
                } => {
                    let group = &self.groups[g];
                    let channel = &group.channels[c];
                    let claim = &channel.scalar_claims[a];
                    let degree = group.view.degree();
                    for plane in 0..E::DEGREE {
                        let basis = E::from_base_fn(|coordinate| {
                            if coordinate == plane {
                                K::one()
                            } else {
                                K::zero()
                            }
                        });
                        for (flat, &tail) in claim.weights.iter().enumerate() {
                            let coefficient = tail * basis;
                            for (digit, &power) in channel.partials.powers.iter().enumerate() {
                                let column = channel.partials.address(
                                    (flat / degree) * E::DEGREE + plane,
                                    digit,
                                    flat % degree,
                                );
                                for coordinate in 0..E::DEGREE {
                                    entry(
                                        start + coordinate,
                                        column,
                                        coefficient.base_coefficient(coordinate) * power,
                                    );
                                }
                            }
                        }
                    }
                    for coordinate in 0..E::DEGREE {
                        rhs(start + coordinate, claim.value.base_coefficient(coordinate));
                    }
                }
            }
        }
    }

    pub fn weights<'p>(
        &'p self,
        rho: &[E],
    ) -> Result<CoefficientRelationWeights<'p, 'a, K, E>, AkitaError> {
        if rho.len() != self.row_num_vars() {
            return Err(invalid("coefficient row point length"));
        }
        Ok(CoefficientRelationWeights {
            plan: self,
            row_weights: EqPolynomial::evals(rho)?,
        })
    }
    pub fn accumulate_columns(&self, rho: &[E], destination: &mut [E]) -> Result<(), AkitaError> {
        self.weights(rho)?.accumulate_columns(destination)
    }
    pub fn evaluate_column_mle(&self, rho: &[E], point: &[E]) -> Result<E, AkitaError> {
        self.weights(rho)?.evaluate_column_mle(point)
    }
    pub fn rhs_at(&self, rho: &[E]) -> Result<E, AkitaError> {
        Ok(self.weights(rho)?.rhs())
    }

    /// Native reference semantics for callers that already hold the original
    /// K witness. This does not accept a replacement E-valued source.
    pub fn residuals(&self, witness: &[K]) -> Result<Vec<K>, AkitaError> {
        if witness.len() != self.domain_len {
            return Err(invalid("coefficient witness length"));
        }
        let mut lhs = vec![K::zero(); self.rows];
        let mut rhs_values = vec![K::zero(); self.rows];
        self.visit_entries(
            |row, column, weight| lhs[row] += witness[column] * weight,
            |row, value| rhs_values[row] += value,
        );
        for (value, rhs) in lhs.iter_mut().zip(rhs_values) {
            *value -= rhs;
        }
        Ok(lhs)
    }
}

/// rho preparation reusable by both dense prover accumulation and the direct
/// matrix-dependent verifier scan. Only the Rbar row equality table is stored.
pub struct CoefficientRelationWeights<'p, 'a, K, E> {
    plan: &'p CoefficientRelationPlan<'a, K, E>,
    row_weights: Vec<E>,
}
impl<K: Field, E: ExtField<K>> CoefficientRelationWeights<'_, '_, K, E> {
    pub fn accumulate_columns(&self, destination: &mut [E]) -> Result<(), AkitaError> {
        if destination.len() != self.plan.domain_len {
            return Err(invalid("coefficient destination length"));
        }
        self.plan.visit_entries(
            |row, column, weight| destination[column] += self.row_weights[row].mul_base(weight),
            |_, _| {},
        );
        Ok(())
    }
    pub fn evaluate_column_mle(&self, point: &[E]) -> Result<E, AkitaError> {
        if point.len() != self.plan.num_vars() {
            return Err(invalid("coefficient endpoint length"));
        }
        let columns = OffsetEqWindow::new(point)?;
        let mut value = E::zero();
        self.plan.visit_entries(
            |row, column, weight| {
                value += (self.row_weights[row] * columns.eval(column)).mul_base(weight)
            },
            |_, _| {},
        );
        Ok(value)
    }
    pub fn rhs(&self) -> E {
        let mut value = E::zero();
        // No matrix scan is needed for public payload and scalar targets.
        for block in &self.plan.manifest {
            let payload = match block.family {
                CoefficientRowFamily::B { group }
                    if self.plan.groups[group].f.stages.is_empty() =>
                {
                    Some(self.plan.groups[group].f.payload)
                }
                CoefficientRowFamily::D if self.plan.h.stages.is_empty() => {
                    Some(self.plan.h.payload)
                }
                CoefficientRowFamily::F { group, stage }
                    if stage + 1 == self.plan.groups[group].f.stages.len() =>
                {
                    Some(self.plan.groups[group].f.payload)
                }
                CoefficientRowFamily::H { stage } if stage + 1 == self.plan.h.stages.len() => {
                    Some(self.plan.h.payload)
                }
                _ => None,
            };
            if let Some(payload) = payload {
                for (index, &coefficient) in payload.iter().enumerate() {
                    value += self.row_weights[block.rows.start + index].mul_base(coefficient);
                }
            }
            if let CoefficientRowFamily::Scalar {
                group,
                channel,
                claim,
            } = block.family
            {
                let scalar = self.plan.groups[group].channels[channel].scalar_claims[claim].value;
                for coordinate in 0..E::DEGREE {
                    value += self.row_weights[block.rows.start + coordinate]
                        .mul_base(scalar.base_coefficient(coordinate));
                }
            }
        }
        value
    }
}

fn validate_chain<K>(
    chain: &CompressionChain<'_, K>,
    mut image_len: usize,
    domain_len: usize,
    used: &mut Vec<Range<usize>>,
    binary: &mut Vec<Range<usize>>,
) -> Result<(), AkitaError> {
    for stage in &chain.stages {
        if stage.digit_bits == 0
            || stage.input.start >= stage.input.end
            || stage.input.end > domain_len
            || stage.input.len() != product(&[image_len, stage.digit_bits])?
            || stage.matrix.input_len() != stage.input.len()
        {
            return Err(invalid("incomplete native compression input or transpose"));
        }
        used.push(stage.input.clone());
        binary.push(stage.input.clone());
        image_len = stage.matrix.output_len();
    }
    if chain.payload.len() != image_len {
        return Err(invalid("compression final payload geometry"));
    }
    Ok(())
}

fn visit_multiplier<K: Field>(coefficients: &[K], mut visit: impl FnMut(usize, usize, K)) {
    let degree = coefficients.len();
    for t in 0..degree {
        for j in 0..degree {
            let coefficient = if j <= t {
                coefficients[t - j]
            } else {
                -coefficients[degree + t - j]
            };
            if !coefficient.is_zero() {
                visit(t, j, coefficient);
            }
        }
    }
}
fn visit_native<K: Field>(
    matrix: &NativeMatrix<'_, K>,
    start: usize,
    visit: impl FnMut(usize, K, usize),
) {
    visit_native_interval(matrix, start, 0..matrix.input_len(), visit);
}
fn visit_native_interval<K: Field>(
    matrix: &NativeMatrix<'_, K>,
    start: usize,
    interval: Range<usize>,
    mut visit: impl FnMut(usize, K, usize),
) {
    let degree = matrix.degree;
    for row in 0..matrix.rows {
        for flat in interval.clone() {
            let column = flat / degree;
            let j = flat % degree;
            for t in 0..degree {
                let weight = if j <= t {
                    matrix.coefficient(row, column, t - j)
                } else {
                    -matrix.coefficient(row, column, degree + t - j)
                };
                if !weight.is_zero() {
                    visit(flat, weight, start + row * degree + t);
                }
            }
        }
    }
}
fn visit_image_target<K: Field>(
    chain: &CompressionChain<'_, K>,
    next_stage: usize,
    image_len: usize,
    start: usize,
    entry: &mut impl FnMut(usize, usize, K),
    rhs: &mut impl FnMut(usize, K),
) {
    if let Some(next) = chain.stages.get(next_stage) {
        let mut power = K::one();
        for digit in 0..next.digit_bits {
            for coefficient in 0..image_len {
                entry(
                    start + coefficient,
                    next.input.start + coefficient + image_len * digit,
                    -power,
                );
            }
            power += power;
        }
    } else {
        for (coefficient, &value) in chain.payload.iter().enumerate() {
            rhs(start + coefficient, value);
        }
    }
}
fn visit_compression_stage<K: Field>(
    chain: &CompressionChain<'_, K>,
    stage: usize,
    start: usize,
    entry: &mut impl FnMut(usize, usize, K),
    rhs: &mut impl FnMut(usize, K),
) {
    let map = &chain.stages[stage];
    visit_native(&map.matrix, start, |flat, weight, row| {
        entry(row, map.input.start + flat, weight)
    });
    visit_image_target(chain, stage + 1, map.matrix.output_len(), start, entry, rhs);
}
