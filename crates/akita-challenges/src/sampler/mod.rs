//! Pure signed-sparse arithmetic and the registered integer acceptance policies.
//! The ordinary transcript owns one bounded whole-level answer, never a seed.

mod op_norm;
mod op_norm_accumulate;
mod position_sample;
mod signed_sparse;

pub(crate) use position_sample::MAX_STACK_RING_DIM;
pub(crate) use signed_sparse::SignedSparseScratch;

use akita_error::AkitaError;
use std::sync::{Arc, LazyLock};
use crate::{OperatorNormRejection, SparseChallengeConfig};
use op_norm::OpNormTable;

/// Position/sign source shared by pure Fisher–Yates and signed arithmetic.
/// Each range call consumes its complete finite fixed slot, including slack.
pub(crate) trait RangeSignSource {
    fn sample_range(&mut self, modulus: usize) -> Result<usize, AkitaError>;
    fn fill_bytes(&mut self, out: &mut [u8]) -> Result<(), AkitaError>;
}

const OP_NORM_PREDICATE_SCALE: u32 = 48;
static D64_SELECTIVE_L2_OP_NORM_TABLE: LazyLock<Result<Arc<OpNormTable>, &'static str>> =
    LazyLock::new(|| {
        let config = crate::D64_SELECTIVE_L2_CHALLENGE_CONFIG;
        let policy = OperatorNormRejection::D64_SELECTIVE_L2;
        let table = OpNormTable::new(
            64, OP_NORM_PREDICATE_SCALE, config.l1_norm() as u64, u64::from(policy.threshold),
        ).map_err(|_| "failed to initialize the D64 selective-L2 operator-norm table")?;
        if !table.strict_threshold_contains_shrunken_subset(
            policy.fractional_bits, policy.root_coordinate_error_units,
            u64::from(policy.threshold), policy.rounding_margin_units,
        ) {
            return Err("the D64 operator-norm table does not contain its certified subset");
        }
        Ok(Arc::new(table))
    });
static D128_SELECTIVE_L2_OP_NORM_TABLE: LazyLock<Result<Arc<OpNormTable>, &'static str>> =
    LazyLock::new(|| {
        let config = crate::D128_SELECTIVE_L2_CHALLENGE_CONFIG;
        let policy = OperatorNormRejection::D128_SELECTIVE_L2;
        let table = OpNormTable::new(
            128, OP_NORM_PREDICATE_SCALE, config.l1_norm() as u64, u64::from(policy.threshold),
        ).map_err(|_| "failed to initialize the D128 selective-L2 operator-norm table")?;
        if !table.strict_threshold_contains_shrunken_subset(
            policy.fractional_bits, policy.root_coordinate_error_units,
            u64::from(policy.threshold), policy.rounding_margin_units,
        ) {
            return Err("the D128 operator-norm table does not contain its certified subset");
        }
        Ok(Arc::new(table))
    });

/// The actual registered fixed-point predicate, reused by decoder and support checks.
#[derive(Clone)]
pub(crate) enum ShellSamplingPolicy {
    Unrestricted,
    OperatorNorm { threshold: u32, table: Arc<OpNormTable> },
}
impl ShellSamplingPolicy {
    pub(crate) fn new(ring_d: usize, cfg: &SparseChallengeConfig,
        rejection: Option<OperatorNormRejection>) -> Result<Self, AkitaError> {
        let Some(rejection) = rejection else { return Ok(Self::Unrestricted); };
        rejection.validate(ring_d, cfg).map_err(|error| AkitaError::InvalidSetup(error.into()))?;
        let table = match rejection {
            OperatorNormRejection::D64_SELECTIVE_L2 => &*D64_SELECTIVE_L2_OP_NORM_TABLE,
            OperatorNormRejection::D128_SELECTIVE_L2 => &*D128_SELECTIVE_L2_OP_NORM_TABLE,
            _ => return Err(AkitaError::InvalidSetup("unsupported operator-norm rejection policy".into())),
        };
        let table = Arc::clone(table.as_ref().map_err(|message| AkitaError::InvalidSetup((*message).into()))?);
        Ok(Self::OperatorNorm { threshold: rejection.threshold, table })
    }

    pub(crate) fn accepts(&self, positions: &[u32], coeffs: &[i8]) -> Result<bool, AkitaError> {
        match self {
            Self::Unrestricted => Ok(true),
            Self::OperatorNorm { threshold, table } =>
                table.accept_strict_parts(positions, coeffs, u64::from(*threshold)),
        }
    }
}
