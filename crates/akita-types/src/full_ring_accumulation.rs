//! Opt-in compact repeated-root installation and checked frontend obligations.
//!
//! Native ownership checks count, source identity, domain and normalization.
//! The consuming application callback must additionally check VM adjacency,
//! original RAM-handle continuity and active-row limits. Opaque installation
//! bytes are transport, not authentication: callers must authenticate the
//! complete setup, schedule, source/decoder/domain plans and event sites.
use crate::full_ring::{DecoderPlan, RawSourceViewPlan, SourceDomainPlan};
use crate::full_ring_root::FullRingReductionSites;
use crate::{Commitment, CommittedGroup, FullRingSetupDescriptor, FullRingSourceId, RingVec};
use akita_error::AkitaError;
use akita_params::{FoldSchedule, OpeningClaimsLayout, RelationWitnessGeometry};
use akita_transcript::native::{E, K};
use akita_transcript::native_strong::{
    FinalReceiverDomainSites, NormalizationSites, SourceDomainSites,
};
use jolt_field::{CanonicalBytes, Zero};
use std::sync::Arc;

fn invalid(message: &str) -> AkitaError {
    AkitaError::InvalidInput(message.into())
}

/// Installed fresh-source prefix. The caller MUST authenticate and admit the
/// source producer, every concrete decoder, required-zero mask and claim bound
/// before construction. Neither nonempty `installation_binding` nor structural
/// validation establishes that admission. The opaque bytes are retained as
/// transport alongside a canonical encoding of the actual structured inputs.
pub struct FullRingFreshSourcePlan {
    domain: SourceDomainPlan,
    decoders: Vec<DecoderPlan<K>>,
    max_claims: usize,
    descriptor: Vec<u8>,
}
impl FullRingFreshSourcePlan {
    pub fn new(
        domain: SourceDomainPlan,
        decoders: Vec<DecoderPlan<K>>,
        max_claims: usize,
        installation_binding: Vec<u8>,
    ) -> Result<Self, AkitaError> {
        if max_claims == 0
            || installation_binding.is_empty()
            || decoders
                .iter()
                .any(|decoder| decoder.view() != domain.view())
        {
            return Err(invalid("fresh-source installation/decoder mismatch"));
        }
        let descriptor =
            fresh_prefix_descriptor(&domain, &decoders, max_claims, &installation_binding)?;
        Ok(Self {
            domain,
            decoders,
            max_claims,
            descriptor,
        })
    }
    pub fn domain(&self) -> &SourceDomainPlan {
        &self.domain
    }
    pub fn decoders(&self) -> &[DecoderPlan<K>] {
        &self.decoders
    }
    pub fn max_claims(&self) -> usize {
        self.max_claims
    }
}

/// Concrete endpoints returned by checked application replay. Decoder indices
/// refer only to the immutable installation; no decoder can arrive in a proof.
pub enum FullRingApplicationClaim {
    Raw {
        point: Vec<E>,
        value: E,
    },
    Ordinary {
        decoder: usize,
        point: Vec<E>,
        value: E,
    },
}
pub struct FullRingFreshSourceObligations {
    pub commitment: CommittedGroup<K>,
    pub claims: Vec<FullRingApplicationClaim>,
}
/// Public order RAM-minus, RAM-plus, Main. Receiver is deliberately absent:
/// it belongs to the native owner and cannot be replaced by the callback.
pub struct FullRingFreshObligations {
    pub sources: [FullRingFreshSourceObligations; 3],
}
pub struct FullRingFreshSourceSites<R, D> {
    pub domain: SourceDomainSites<R, D>,
    pub normalization: NormalizationSites<R, D>,
}
pub struct FullRingUpdateSites<R, D, S> {
    pub receiver_domain: FinalReceiverDomainSites<R, D>,
    pub receiver_normalization: NormalizationSites<R, D>,
    pub fresh: [FullRingFreshSourceSites<R, D>; 3],
    pub reduction: FullRingReductionSites<R, D, S>,
}

/// Validated native geometry, not an installation/security certificate. The
/// original installed matrix values and authenticated producer descriptors
/// remain prerequisites. Root occurrence serials never become template levels.
/// Each exact ordered update has its own admitted source-domain/decoder table;
/// Arc sharing preserves immutable reusable RAM/decoder metadata without
/// copying it. The table does not establish that any per-update inactive mask
/// matches the VM's active length: the checked application must establish that.
pub struct FullRingAccumulationPlan {
    setup: FullRingSetupDescriptor,
    schedule: FoldSchedule,
    layout: OpeningClaimsLayout,
    fresh: Vec<[Arc<FullRingFreshSourcePlan>; 3]>,
    zero_receiver: CommittedGroup<K>,
}
impl FullRingAccumulationPlan {
    pub fn new(
        setup: &FullRingSetupDescriptor,
        schedule: &FoldSchedule,
        layout: OpeningClaimsLayout,
        fresh: Vec<[Arc<FullRingFreshSourcePlan>; 3]>,
    ) -> Result<Self, AkitaError> {
        FullRingUpdateCount::new(fresh.len())?;
        setup.validate_canonical_compact()?;
        schedule.validate_structure()?;
        schedule.root.params.validate_full_ring_execution(2)?;
        schedule.root.params.validate_opening_batch(&layout)?;
        if layout.num_groups() != 4
            || layout.root_group_order()? != [3, 0, 1, 2]
            || schedule.root.output_witness_len != 1 << 22
            || schedule.recursive_folds.len() != 3
        {
            return Err(invalid(
                "accumulation requires the installed compact root and three-fold suffix",
            ));
        }
        let receiver = setup.source(FullRingSourceId(0))?;
        for (index, &source_id) in [0, 2, 2, 1].iter().enumerate() {
            let source = setup.source(FullRingSourceId(source_id))?;
            let group = schedule.root.params.group_params_geometry(&layout, index)?;
            if source.polynomials != layout.group_layout(index)?.num_polynomials()
                || source.blocks != group.num_live_blocks()
                || source.positions != group.num_positions_per_block()
                || source.digit_depth != group.num_digits_inner()
                || source.rank != group.a_rows_len()
            {
                return Err(invalid(
                    "compact root profile differs from installed source",
                ));
            }
        }
        for update in &fresh {
            for (index, prefix) in update.iter().enumerate() {
                let id = FullRingSourceId([2, 2, 1][index]);
                if prefix.domain.view() != &setup.source(id)?.raw_view {
                    return Err(invalid(
                        "fresh source domain differs from installed original view",
                    ));
                }
                let expected_alphabet = if index == 2 {
                    crate::full_ring::SourceAlphabet::BooleanWholeGrid
                } else {
                    crate::full_ring::SourceAlphabet::BalancedBase32
                };
                if prefix.domain.alphabet() != expected_alphabet {
                    return Err(invalid(
                        "fresh source alphabet differs from its compact role",
                    ));
                }
            }
        }
        let mut input_len: usize = 1 << 22;
        for (index, fold) in schedule.recursive_folds.iter().enumerate() {
            if fold.input_witness_len != input_len || fold.output_witness_len != input_len / 2 {
                return Err(invalid("compact shrinking suffix extent"));
            }
            fold.params.validate_full_ring_execution(2)?;
            let suffix_layout = OpeningClaimsLayout::new(input_len.trailing_zeros() as usize, 1)?;
            let source = setup.source(
                *setup
                    .level(index + 1)?
                    .sources
                    .first()
                    .ok_or_else(|| invalid("missing suffix source"))?,
            )?;
            let group = fold.params.group_params_geometry(&suffix_layout, 0)?;
            if source.raw_view.physical_len() != input_len
                || source.blocks != group.num_live_blocks()
                || source.positions != group.num_positions_per_block()
                || source.digit_depth != group.num_digits_inner()
                || source.rank != group.a_rows_len()
            {
                return Err(invalid(
                    "compact suffix profile differs from installed source",
                ));
            }
            input_len /= 2;
        }
        if receiver.raw_view.physical_len() != 1 << 22
            || receiver.raw_view.polynomials() != 1
            || receiver.raw_view.digits() != 1
            || schedule.terminal.blocks.live_blocks != 8
            || schedule.terminal.blocks.positions_per_block != 512
            || schedule.terminal.input_witness_len != 1 << 19
            || schedule.terminal.response_codec
                != akita_params::TerminalResponseCodec::full_ring(8)?
        {
            return Err(invalid("compact receiver or terminal geometry"));
        }
        let profile = schedule.root.params.group_params(&layout, 0)?.profile;
        let geometry = RelationWitnessGeometry::for_level(&schedule.root.params, &layout, 2)?;
        let compression = geometry.rhs_layout().compression_plan_for_group(0)?;
        let payload_len = compression.terminal_coefficients();
        let payload_dimension = compression
            .maps()
            .last()
            .ok_or_else(|| invalid("receiver commitment compression has no terminal map"))?
            .ring_dimension();
        // Canonical zero uses zero A/B images and all-zero F digits. Its live
        // public payload has the terminal F-map dimension, just like the CPU
        // producer; that dimension is not the original source's native128.
        // Later receiver paths remain noncanonical and are never rewritten.
        let zero_receiver = CommittedGroup::new(
            profile,
            Commitment::new(RingVec::from_coeffs_with_ring_dim(
                vec![K::zero(); payload_len],
                payload_dimension,
            )?),
        );
        Ok(Self {
            setup: setup.clone(),
            schedule: schedule.clone(),
            layout,
            fresh,
            zero_receiver,
        })
    }
    pub fn validate(
        &self,
        setup: &FullRingSetupDescriptor,
        schedule: &FoldSchedule,
    ) -> Result<(), AkitaError> {
        if setup != &self.setup
            || schedule.canonical_descriptor_bytes() != self.schedule.canonical_descriptor_bytes()
        {
            return Err(invalid("accumulation installation or schedule replacement"));
        }
        Ok(())
    }
    pub fn layout(&self) -> &OpeningClaimsLayout {
        &self.layout
    }
    pub fn num_updates(&self) -> usize {
        self.fresh.len()
    }
    pub fn fresh_sources(
        &self,
        update: usize,
    ) -> Result<&[Arc<FullRingFreshSourcePlan>; 3], AkitaError> {
        self.fresh
            .get(update)
            .ok_or_else(|| invalid("uninstalled root occurrence"))
    }
    pub fn zero_receiver(&self) -> &CommittedGroup<K> {
        &self.zero_receiver
    }
    pub fn receiver_view(&self) -> Result<&RawSourceViewPlan, AkitaError> {
        Ok(&self.setup.source(FullRingSourceId(0))?.raw_view)
    }
}

/// Exact bounded count shared by native owners. Successful roots alone advance
/// it; final normalization and suffix execution require completion.
pub struct FullRingUpdateCount {
    total: usize,
    completed: usize,
}
impl FullRingUpdateCount {
    pub fn new(total: usize) -> Result<Self, AkitaError> {
        if !(1..=16).contains(&total) {
            return Err(invalid("accumulation count must be 1..=16"));
        }
        Ok(Self {
            total,
            completed: 0,
        })
    }
    pub fn total(&self) -> usize {
        self.total
    }
    pub fn completed(&self) -> usize {
        self.completed
    }
    pub fn require_update(&self) -> Result<(), AkitaError> {
        if self.completed >= self.total {
            return Err(invalid("all declared roots already completed"));
        }
        Ok(())
    }
    pub fn complete_update(&mut self) -> Result<(), AkitaError> {
        self.require_update()?;
        self.completed += 1;
        Ok(())
    }
    pub fn require_complete(&self) -> Result<(), AkitaError> {
        if self.completed != self.total {
            return Err(invalid("not all declared roots completed"));
        }
        Ok(())
    }
}

fn push_size(bytes: &mut Vec<u8>, value: usize) -> Result<(), AkitaError> {
    bytes.extend_from_slice(
        &u64::try_from(value)
            .map_err(|_| invalid("binding length overflow"))?
            .to_le_bytes(),
    );
    Ok(())
}
/// Called once, before first frontend replay, on the first root instance site.
pub fn full_ring_accumulation_binding(
    plan: &FullRingAccumulationPlan,
    total: usize,
) -> Result<Vec<u8>, AkitaError> {
    FullRingUpdateCount::new(total)?;
    if total != plan.num_updates() {
        return Err(invalid(
            "declared roots differ from the installed per-update table",
        ));
    }
    let mut bytes = crate::full_ring_root::full_ring_instance_binding(&plan.setup, &plan.schedule)?;
    push_size(&mut bytes, total)?;
    for update in &plan.fresh {
        for (prefix, source) in update.iter().zip([2, 2, 1]) {
            let binding = full_ring_fresh_source_binding(prefix, FullRingSourceId(source), &[])?;
            push_size(&mut bytes, binding.len())?;
            bytes.extend_from_slice(&binding);
        }
    }
    Ok(bytes)
}
fn fresh_prefix_descriptor(
    domain: &SourceDomainPlan,
    decoders: &[DecoderPlan<K>],
    max_claims: usize,
    installation_binding: &[u8],
) -> Result<Vec<u8>, AkitaError> {
    let mut bytes = Vec::new();
    // Versioned cutover: balanced sources now use (5,5,3), not a direct
    // alphabet polynomial. Presence is installation data, never proof input.
    bytes.extend_from_slice(b"akita/fresh-source-prefix/v2");
    bytes.push(u8::from(domain.has_physical_zeros()));
    push_size(&mut bytes, installation_binding.len())?;
    bytes.extend_from_slice(installation_binding);
    push_size(&mut bytes, max_claims)?;
    let view = domain.view();
    for size in [
        view.degree(),
        view.polynomials(),
        view.blocks(),
        view.positions(),
        view.digits(),
    ] {
        push_size(&mut bytes, size)?;
    }
    if let Some(permutation) = view.permutation() {
        bytes.push(1);
        push_size(&mut bytes, permutation.len())?;
        for &index in permutation {
            push_size(&mut bytes, index)?;
        }
    } else {
        bytes.push(0);
    }
    bytes.push(match domain.alphabet() {
        crate::full_ring::SourceAlphabet::BooleanWholeGrid => 0,
        crate::full_ring::SourceAlphabet::BalancedBase32 => 1,
    });
    let support = domain.required_physical_zeros();
    push_size(&mut bytes, support.domain_len())?;
    push_size(&mut bytes, support.intervals().len())?;
    for range in support.intervals() {
        push_size(&mut bytes, range.start)?;
        push_size(&mut bytes, range.end)?;
    }
    push_size(&mut bytes, decoders.len())?;
    for decoder in decoders {
        let descriptor = decoder.canonical_descriptor_bytes()?;
        push_size(&mut bytes, descriptor.len())?;
        bytes.extend_from_slice(&descriptor);
    }
    Ok(bytes)
}

/// Binds the installed original source and actual public payload, including
/// the literal decoder maps, affine constants and domain supports. Binding is
/// not authentication of the caller's installation or application semantics.
pub fn full_ring_fresh_source_binding(
    plan: &FullRingFreshSourcePlan,
    source: FullRingSourceId,
    public: &[K],
) -> Result<Vec<u8>, AkitaError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&source.0.to_le_bytes());
    push_size(&mut bytes, plan.descriptor.len())?;
    bytes.extend_from_slice(&plan.descriptor);
    push_size(&mut bytes, public.len())?;
    for value in public {
        let mut atom = [0; 16];
        value.to_bytes_le(&mut atom);
        bytes.extend_from_slice(&atom);
    }
    Ok(bytes)
}

/// Authenticate the particular installed decoder selected by each application
/// claim before the normalization alpha draw. Endpoint values themselves are
/// additionally bound by the existing normalization transcript driver.
pub fn full_ring_fresh_obligation_binding(
    plan: &FullRingFreshSourcePlan,
    source: FullRingSourceId,
    public: &[K],
    claims: &[FullRingApplicationClaim],
) -> Result<Vec<u8>, AkitaError> {
    if claims.is_empty() || claims.len() > plan.max_claims {
        return Err(invalid("fresh application normalization claim count"));
    }
    let mut bytes = full_ring_fresh_source_binding(plan, source, public)?;
    push_size(&mut bytes, claims.len())?;
    for claim in claims {
        match claim {
            FullRingApplicationClaim::Raw { point, .. } => {
                if point.len() != plan.domain.view().num_vars() {
                    return Err(invalid("application raw point has wrong source dimension"));
                }
                bytes.push(0);
            }
            FullRingApplicationClaim::Ordinary { decoder, point, .. } => {
                let installed = plan
                    .decoders
                    .get(*decoder)
                    .ok_or_else(|| invalid("uninstalled application decoder"))?;
                if point.len() != installed.decoded_len().trailing_zeros() as usize {
                    return Err(invalid("application decoded point has wrong dimension"));
                }
                bytes.push(1);
                push_size(&mut bytes, *decoder)?;
            }
        }
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::full_ring::{CoefficientSupport, DecoderTerm, SourceAlphabet};
    use jolt_field::Ring;
    #[test]
    fn exact_bounded_roots_gate_finalization() {
        assert!(FullRingUpdateCount::new(0).is_err());
        assert!(FullRingUpdateCount::new(17).is_err());
        for total in 1..=16 {
            let mut count = FullRingUpdateCount::new(total).unwrap();
            for index in 0..total {
                assert_eq!(count.completed(), index);
                assert!(count.require_complete().is_err());
                count.complete_update().unwrap();
            }
            count.require_complete().unwrap();
            assert!(count.require_update().is_err());
            assert!(count.complete_update().is_err());
            assert_eq!(count.completed(), total);
        }
    }

    #[test]
    fn literal_decoder_and_ordered_selection_are_bound() {
        let view = RawSourceViewPlan::new(2, 1, 1, 1, 1).unwrap();
        let domain = SourceDomainPlan::new(
            view.clone(),
            SourceAlphabet::BooleanWholeGrid,
            CoefficientSupport::new(2, Vec::new()).unwrap(),
        )
        .unwrap();
        let decoder =
            |scale| DecoderPlan::positional(&view, 0, K::from_u64(32), K::from_u64(scale)).unwrap();
        let make = |scale| {
            FullRingFreshSourcePlan::new(
                domain.clone(),
                vec![decoder(1), decoder(scale)],
                2,
                vec![9],
            )
            .unwrap()
        };
        let first = make(2);
        let changed = make(3);
        let source = FullRingSourceId(2);
        let public = [K::zero()];
        assert_ne!(
            full_ring_fresh_source_binding(&first, source, &public).unwrap(),
            full_ring_fresh_source_binding(&changed, source, &public).unwrap(),
        );
        let claims = |decoder| {
            vec![FullRingApplicationClaim::Ordinary {
                decoder,
                point: vec![E::zero()],
                value: E::zero(),
            }]
        };
        let binding =
            full_ring_fresh_obligation_binding(&first, source, &public, &claims(0)).unwrap();
        assert_ne!(
            binding,
            full_ring_fresh_obligation_binding(&first, source, &public, &claims(1)).unwrap()
        );
        assert!(full_ring_fresh_obligation_binding(&first, source, &public, &claims(2)).is_err());
        assert!(full_ring_fresh_obligation_binding(&first, source, &public, &[]).is_err());
        let malformed = [FullRingApplicationClaim::Raw {
            point: vec![],
            value: E::zero(),
        }];
        assert!(full_ring_fresh_obligation_binding(&first, source, &public, &malformed).is_err());

        let sparse = |coefficient, constant| {
            DecoderPlan::sparse(
                &view,
                2,
                vec![DecoderTerm {
                    raw_index: 1,
                    decoded_index: 0,
                    coefficient: K::from_u64(coefficient),
                }],
                vec![(1, K::from_u64(constant))],
            )
            .unwrap()
            .canonical_descriptor_bytes()
            .unwrap()
        };
        assert_ne!(sparse(1, 2), sparse(2, 2));
        assert_ne!(sparse(1, 2), sparse(1, 3));
    }
}
