//! Bounded repeated roots on the existing complete reduction, never a second engine.
use super::*;
use akita_transcript::native_strong::NativeTranscriptBorrow;
use akita_types::full_ring::NormalizationClaim;
use akita_types::full_ring_accumulation::{
    full_ring_accumulation_binding, full_ring_fresh_obligation_binding, FullRingAccumulationPlan,
    FullRingApplicationClaim, FullRingFreshObligations, FullRingUpdateCount, FullRingUpdateSites,
};
use jolt_field::Zero;
use std::sync::Arc;

/// Fresh original handles paired with concrete checked frontend endpoints.
/// Public order is RAM-minus, RAM-plus, Main. The receiver cannot be supplied.
pub struct FullRingFreshProverObligations<H> {
    pub handles: [H; 3],
    pub obligations: FullRingFreshObligations,
}

/// Start the first root with the registered actual zero source and the actual
/// compiler. The callback must enforce application VM/RAM adjacency and active
/// lengths; native checks source ownership, full domains and normalization.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn begin_full_ring_accumulation<'a, Cfg, B, T, R, D, X>(
    setup: &FullRingSetupDescriptor,
    schedules: &TrustedScheduleCatalog<Cfg>,
    selection: akita_params::OpeningScheduleSelection,
    plan: Arc<FullRingAccumulationPlan>,
    total: usize,
    backend: &'a B,
    zero_receiver: B::CommitmentHandle,
    mut transcript: T,
    mut proof: Vec<u8>,
    sites: &FullRingUpdateSites<R, D, T::ShellSite>,
    frontend: impl FnOnce(
        NativeTranscriptBorrow<'_, T, R, D>,
        &mut Vec<u8>,
        usize,
        &RingVec<K>,
    ) -> Result<FullRingFreshProverObligations<B::CommitmentHandle>, X>,
) -> Result<PendingFullRingProver<'a, B, T, R, D>, X>
where
    Cfg: CommitmentConfig<Field = K, ExtField = E>,
    B: FullRingProverBackend<K, E>,
    T: NativeStrongTranscript<R, D>,
    X: From<AkitaError>,
{
    let result = (|| -> Result<_, X> {
        let transcript = &mut transcript;
        akita_config::validate_config_policy::<Cfg>()?;
        if Cfg::opening_family() != PlannerOpeningFamily::FullRingE2InnerRecursive
            || T::FAMILY != NativeCompilerFamily::InnerH1610
        {
            return Err(
                invalid("accumulation requires the registered inner compiler family").into(),
            );
        }
        let mut count = FullRingUpdateCount::new(total)?;
        let resolved = schedules.resolve_selection(selection)?;
        resolved.validate_opening_layout(plan.layout())?;
        let schedule = resolved.schedule();
        plan.validate(setup, schedule)?;
        if total != plan.num_updates() {
            return Err(invalid("declared roots differ from prefix-plan table").into());
        }
        let session =
            backend.begin_full_ring_accumulation(setup, schedules, schedule, &plan, total)?;
        let scope = ProofScope::admitted(backend, session);
        let context = backend.proof_context(scope.session(), 0)?;
        let zero = plan.zero_receiver();
        let material = backend.validate_commitment(
            scope.session(),
            &context.for_group(0),
            &zero_receiver,
            zero.profile(),
            zero.commitment(),
        )?;
        if backend.full_ring_source_id(&zero_receiver)? != akita_types::FullRingSourceId(0) {
            return Err(invalid("initial receiver is not the registered receiver contract").into());
        }
        backend.with_full_ring_source(
            scope.session(),
            &context.for_group(0),
            &zero_receiver,
            |source| {
                if source.len() != plan.receiver_view()?.physical_len() {
                    return Err(invalid("zero receiver extent"));
                }
                for index in 0..source.len() {
                    if !source.coefficient(index)?.is_zero() {
                        return Err(invalid("initial receiver source is not zero"));
                    }
                }
                Ok(())
            },
        )?;
        transcript.public(
            &sites.reduction.instance,
            1,
            &full_ring_accumulation_binding(&plan, total)?,
        )?;
        let fresh = frontend(
            NativeTranscriptBorrow::new(transcript),
            &mut proof,
            0,
            zero.commitment().rows(),
        )?;
        let receiver = FullRingRawClaim {
            point: FullRingRawPoint::new(
                plan.receiver_view()?,
                vec![E::zero(); plan.receiver_view()?.num_vars()],
            )?,
            value: E::zero(),
        };
        let result = update_root(
            backend,
            scope.session(),
            schedule,
            &plan,
            0,
            OpeningSource::Commitment(&zero_receiver),
            material,
            zero.commitment().rows().clone(),
            receiver,
            fresh,
            transcript,
            &mut proof,
            sites,
        )?;
        count.complete_update()?;
        Ok((scope, schedule.clone(), result, count))
    })();
    let (scope, schedule, result, count) = result.inspect_err(|_| {
        transcript.reject();
    })?;
    Ok(PendingFullRingProver {
        scope,
        backend,
        schedule,
        transcript,
        proof,
        witness: result.witness,
        material: result.material,
        binding: result.binding,
        point: result.point,
        value: result.value,
        domain: result.domain,
        final_normalized: false,
        next_recursive: 0,
        context: result.context,
        accumulation: Some((plan, count)),
        marker: PhantomData,
    })
}

#[allow(clippy::too_many_arguments)]
fn update_root<B, T, R, D>(
    backend: &B,
    session: &B::ProofSessionHandle,
    schedule: &FoldSchedule,
    plan: &FullRingAccumulationPlan,
    update: usize,
    receiver: OpeningSource<'_, B::CommitmentHandle, B::WitnessHandle>,
    receiver_material: B::CommitmentMaterialHandle,
    receiver_public: RingVec<K>,
    receiver_claim: FullRingRawClaim,
    fresh: FullRingFreshProverObligations<B::CommitmentHandle>,
    transcript: &mut T,
    proof: &mut Vec<u8>,
    sites: &FullRingUpdateSites<R, D, T::ShellSite>,
) -> Result<ReductionOutput<B>, AkitaError>
where
    B: FullRingProverBackend<K, E>,
    T: NativeStrongTranscript<R, D>,
{
    let context = backend.proof_context(session, 0)?;
    let mut sources = vec![receiver];
    let mut materials = vec![receiver_material];
    let mut public = vec![receiver_public];
    let mut raw = vec![receiver_claim];
    for (index, ((handle, obligations), prefix)) in fresh
        .handles
        .iter()
        .zip(&fresh.obligations.sources)
        .zip(plan.fresh_sources(update)?)
        .enumerate()
    {
        let group_index = index + 1;
        let parameters = schedule
            .root
            .params
            .group_params(plan.layout(), group_index)?;
        if obligations.commitment.profile() != &parameters.profile {
            return Err(invalid(
                "fresh original commitment profile differs from registered occurrence",
            ));
        }
        materials.push(backend.validate_commitment(
            session,
            &context.for_group(group_index),
            handle,
            &parameters.profile,
            obligations.commitment.commitment(),
        )?);
        let source_id = backend.full_ring_source_id(handle)?;
        if source_id != akita_types::FullRingSourceId([2, 2, 1][index]) {
            return Err(invalid(
                "fresh original source contract differs from occurrence",
            ));
        }
        let binding = full_ring_fresh_obligation_binding(
            prefix,
            source_id,
            obligations.commitment.commitment().rows().coeffs(),
            &obligations.claims,
        )?;
        let (point, value) = backend.with_full_ring_source(
            session,
            &context.for_group(group_index),
            handle,
            |source| {
                let (domain_point, domain_value) =
                    super::super::full_ring_strong::prove_full_ring_source_domain(
                        transcript,
                        &sites.fresh[index].domain,
                        &binding,
                        source,
                        prefix.domain(),
                        proof,
                    )?;
                let mut claims = Vec::with_capacity(obligations.claims.len() + 1);
                for claim in &obligations.claims {
                    claims.push(match claim {
                        FullRingApplicationClaim::Raw { point, value } => NormalizationClaim::Raw {
                            point,
                            value: *value,
                        },
                        FullRingApplicationClaim::Ordinary {
                            decoder,
                            point,
                            value,
                        } => NormalizationClaim::Ordinary {
                            decoder: prefix
                                .decoders()
                                .get(*decoder)
                                .ok_or_else(|| invalid("fresh decoder index"))?,
                            point,
                            value: *value,
                        },
                    });
                }
                claims.push(NormalizationClaim::Raw {
                    point: &domain_point,
                    value: domain_value,
                });
                super::super::full_ring_strong::prove_full_ring_normalization(
                    transcript,
                    &sites.fresh[index].normalization,
                    &binding,
                    source,
                    prefix.domain().view(),
                    claims,
                    proof,
                )
            },
        )?;
        raw.push(FullRingRawClaim {
            point: FullRingRawPoint::new(prefix.domain().view(), point)?,
            value,
        });
        sources.push(OpeningSource::Commitment(handle));
        public.push(obligations.commitment.commitment().rows().clone());
    }
    reduce(
        backend,
        session,
        transcript,
        proof,
        0,
        &schedule.root.params,
        plan.layout(),
        sources,
        materials,
        public,
        raw,
        schedule.root.output_witness_len,
        successor_parameters(schedule, 0),
        &sites.reduction,
    )
}

impl<'a, B, T, R, D> PendingFullRingProver<'a, B, T, R, D>
where
    B: FullRingProverBackend<K, E>,
    T: NativeStrongTranscript<R, D>,
{
    /// A consuming application seam: failures abort this scope and poison this
    /// compiler. The application must check original RAM/private handle equality,
    /// VM boundaries and active lengths before returning concrete fresh endpoints.
    /// The prior receiver and its pending opening are inserted internally.
    pub fn update_with_frontend<X: From<AkitaError>>(
        mut self,
        sites: &FullRingUpdateSites<R, D, T::ShellSite>,
        frontend: impl FnOnce(
            NativeTranscriptBorrow<'_, T, R, D>,
            &mut Vec<u8>,
            usize,
            &RingVec<K>,
        ) -> Result<FullRingFreshProverObligations<B::CommitmentHandle>, X>,
    ) -> Result<Self, X> {
        let result = (|| -> Result<_, X> {
            let transcript = &mut self.transcript;
            let (plan, count) = self
                .accumulation
                .as_ref()
                .ok_or_else(|| invalid("ordinary proof is not an accumulation run"))?;
            count.require_update()?;
            if self.final_normalized || self.next_recursive != 0 {
                return Err(invalid("root update after finalization").into());
            }
            let NextWitnessBindingMessage::OuterPayload(public) = &self.binding else {
                return Err(invalid("root update cannot consume an A-only terminal").into());
            };
            let fresh = frontend(
                NativeTranscriptBorrow::new(transcript),
                &mut self.proof,
                count.completed(),
                public,
            )?;
            let binding = akita_types::full_ring_root::full_ring_domain_binding(
                &base_bytes(public.coeffs())?,
                &self.domain,
            )?;
            let source = self.backend.full_ring_witness_source(&self.witness)?;
            let (domain_point, domain_value) =
                super::super::full_ring_strong::prove_full_ring_final_receiver_domain(
                    transcript,
                    &sites.receiver_domain,
                    &binding,
                    source,
                    &self.domain,
                    &mut self.proof,
                )?;
            let (point, value) = super::super::full_ring_strong::prove_full_ring_normalization(
                transcript,
                &sites.receiver_normalization,
                &binding,
                source,
                self.domain.view(),
                vec![
                    NormalizationClaim::Raw {
                        point: &self.point,
                        value: self.value,
                    },
                    NormalizationClaim::Raw {
                        point: &domain_point,
                        value: domain_value,
                    },
                ],
                &mut self.proof,
            )?;
            update_root(
                self.backend,
                self.scope.session(),
                &self.schedule,
                plan,
                count.completed(),
                OpeningSource::Witness(&self.witness),
                self.material,
                public.clone(),
                FullRingRawClaim {
                    point: FullRingRawPoint::new(self.domain.view(), point)?,
                    value,
                },
                fresh,
                transcript,
                &mut self.proof,
                sites,
            )
            .map_err(X::from)
        })();
        let result = result.inspect_err(|_| {
            self.transcript.reject();
        })?;
        guarded(&mut self.transcript, |_| {
            self.backend.release_witness_handle(self.witness)?;
            self.accumulation
                .as_mut()
                .ok_or_else(|| invalid("lost accumulation owner"))?
                .1
                .complete_update()
        })?;
        self.witness = result.witness;
        self.material = result.material;
        self.binding = result.binding;
        self.point = result.point;
        self.value = result.value;
        self.domain = result.domain;
        self.context = result.context;
        Ok(self)
    }
}
