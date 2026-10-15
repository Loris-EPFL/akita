//! Signed coefficient storage for shared commitment and opening kernels.

use crate::commitment::{AcceptedReach, SignedBlockDecoder};
use crate::sources::block_digits::{
    BlockDigits, DigitPlanes, DigitShape, I16Rows, I8Rows, RingValueSource, RingValues,
};
use crate::sources::poly_helpers::{I8DigitSpan, SignedDigitBasis};
use akita_algebra::ring::cyclotomic::BalancedDecomposePow2Params;
use akita_error::{checked, AkitaError};
use akita_params::SignedDigitKernel;
use jolt_field::{CanonicalEncoding, Field};
use std::marker::PhantomData;
use std::ops::Range;

mod kernels;
mod owned;
pub use owned::SignedBlockPoly;

pub(crate) struct I8Slice<'a>(pub(crate) &'a [i8]);
pub(crate) struct I16Slice<'a>(pub(crate) &'a [i16]);
pub(crate) struct Decoded<'a>(pub(crate) &'a dyn SignedBlockDecoder);

pub(crate) trait SignedStorage: Sync {
    type Value: Copy + Default + Into<i64> + Send + Sync;
    const NEGATIVE_ABS: u128;
    const POSITIVE: u128;
    fn load(&self, start: usize, out: &mut [Self::Value], extrema: &mut Option<Scan>);
    fn copy_map<O>(
        &self,
        start: usize,
        raw: &mut [Self::Value],
        out: &mut [O],
        scan: &mut Option<Scan>,
        map: impl Fn(Self::Value) -> O,
    );
    fn borrowed_i8<const D: usize>(&self, _rings: Range<usize>) -> Option<&[[i8; D]]> {
        None
    }
}

pub(crate) struct Scan {
    min: i64,
    max: i64,
    negative_abs: u128,
    positive: u128,
    accepted: Option<AcceptedReach>,
}

fn track(scan: &mut Option<Scan>, value: i64) {
    if let Some(scan) = scan {
        scan.min = scan.min.min(value);
        scan.max = scan.max.max(value);
        if let Some(bounds) = scan.accepted {
            let (negative_abs, positive) = bounds.centered(value);
            scan.negative_abs = scan.negative_abs.max(negative_abs);
            scan.positive = scan.positive.max(positive);
        }
    }
}

impl AcceptedReach {
    pub(crate) fn centered(self, value: i64) -> (u128, u128) {
        let magnitude = u128::from(value.unsigned_abs());
        let magnitude = if magnitude < self.modulus {
            magnitude
        } else {
            magnitude % self.modulus
        };
        let canonical = if value < 0 && magnitude != 0 {
            self.modulus - magnitude
        } else {
            magnitude
        };
        if canonical > self.centering_threshold {
            (self.modulus - canonical, 0)
        } else {
            (0, canonical)
        }
    }

    fn preserves_carrier(self, negative_abs: u128, positive: u128) -> bool {
        self.centering_threshold >= positive
            && self.modulus - self.centering_threshold > negative_abs
    }

    fn covers_carrier(self, negative_abs: u128, positive: u128) -> bool {
        if self.modulus <= negative_abs || self.centering_threshold >= self.modulus {
            return false;
        }
        let negative_start = self.modulus - negative_abs;
        let positive_reach = positive.min(self.centering_threshold).max(
            if negative_start <= self.centering_threshold {
                self.centering_threshold
            } else {
                0
            },
        );
        let negative_reach = if positive > self.centering_threshold {
            self.modulus - self.centering_threshold - 1
        } else {
            negative_abs.min(self.modulus - self.centering_threshold - 1)
        };
        self.negative_abs
            .is_none_or(|bound| bound >= negative_reach)
            && self.positive.is_none_or(|bound| bound >= positive_reach)
    }
}

macro_rules! slice_storage {
    ($storage:ident, $value:tt, $negative:expr, $positive:expr) => {
        impl SignedStorage for $storage<'_> {
            type Value = $value;
            const NEGATIVE_ABS: u128 = $negative;
            const POSITIVE: u128 = $positive;
            fn load(&self, start: usize, out: &mut [Self::Value], extrema: &mut Option<Scan>) {
                let len = out.len();
                for (dst, &src) in out.iter_mut().zip(&self.0[start..start + len]) {
                    *dst = src;
                    track(extrema, i64::from(src));
                }
            }
            fn copy_map<O>(&self, start: usize, _raw: &mut [Self::Value], out: &mut [O], scan: &mut Option<Scan>, map: impl Fn(Self::Value) -> O) {
                let len = out.len();
                for (dst, &src) in out.iter_mut().zip(&self.0[start..start + len]) {
                    track(scan, i64::from(src));
                    *dst = map(src);
                }
            }
            slice_storage!(@borrow $value);
        }
    };
    (@borrow i8) => {
        fn borrowed_i8<const D: usize>(&self, rings: Range<usize>) -> Option<&[[i8; D]]> {
            self.0.as_chunks::<D>().0.get(rings)
        }
    };
    (@borrow i16) => {};
}
slice_storage!(I8Slice, i8, 128, 127);
slice_storage!(I16Slice, i16, 32768, 32767);

impl SignedStorage for Decoded<'_> {
    type Value = i8;
    const NEGATIVE_ABS: u128 = 128;
    const POSITIVE: u128 = 127;
    fn load(&self, start: usize, out: &mut [i8], extrema: &mut Option<Scan>) {
        self.0.decode_i8(start, out);
        if extrema.is_some() {
            for &value in out.iter() {
                track(extrema, i64::from(value));
            }
        }
    }
    fn copy_map<O>(
        &self,
        start: usize,
        raw: &mut [i8],
        out: &mut [O],
        scan: &mut Option<Scan>,
        map: impl Fn(i8) -> O,
    ) {
        let raw = &mut raw[..out.len()];
        self.0.decode_i8(start, raw);
        for (dst, &src) in out.iter_mut().zip(raw.iter()) {
            track(scan, i64::from(src));
            *dst = map(src);
        }
    }
}

pub(crate) struct SignedBlockDigits<'a, F, St, const D: usize> {
    storage: St,
    len: usize,
    num_rings: usize,
    accepted: Option<AcceptedReach>,
    check_required: bool,
    marker: PhantomData<&'a F>,
}

impl<'a, F, St, const D: usize> SignedBlockDigits<'a, F, St, D>
where
    F: Field + CanonicalEncoding,
    St: SignedStorage,
{
    pub(crate) fn new(
        storage: St,
        num_vars: usize,
        len: usize,
        accepted: Option<AcceptedReach>,
    ) -> Result<Self, AkitaError> {
        if D == 0 || !D.is_power_of_two() {
            return Err(AkitaError::InvalidInput(
                "signed block ring dimension must be a power of two".into(),
            ));
        }
        let domain = checked::pow2(num_vars)
            .ok_or_else(|| AkitaError::InvalidInput("signed block extent overflow".into()))?;
        if len > domain {
            return Err(AkitaError::InvalidInput(
                "signed block length exceeds its domain".into(),
            ));
        }
        let check_required =
            accepted.is_some_and(|bounds| !bounds.covers_carrier(St::NEGATIVE_ABS, St::POSITIVE));
        Ok(Self {
            storage,
            len,
            num_rings: domain.div_ceil(D),
            accepted,
            check_required,
            marker: PhantomData,
        })
    }

    fn scan(&self, digits: bool) -> Option<Scan> {
        (self.check_required || digits).then_some(Scan {
            min: 0,
            max: 0,
            negative_abs: 0,
            positive: 0,
            accepted: self.accepted.filter(|bounds| {
                self.check_required && !bounds.preserves_carrier(St::NEGATIVE_ABS, St::POSITIVE)
            }),
        })
    }

    fn check(&self, scan: &Option<Scan>) -> Result<(), AkitaError> {
        if let (Some(scan), Some(bounds)) = (scan, self.accepted.filter(|_| self.check_required)) {
            let (negative_abs, positive) = if scan.accepted.is_some() {
                (scan.negative_abs, scan.positive)
            } else {
                (u128::from(scan.min.unsigned_abs()), scan.max as u128)
            };
            if bounds
                .negative_abs
                .is_some_and(|bound| negative_abs > bound)
                || bounds.positive.is_some_and(|bound| positive > bound)
            {
                return Err(AkitaError::InvalidInput(
                    "committed source exceeds the scheduled coefficient bound".into(),
                ));
            }
        }
        Ok(())
    }

    fn single_fits(&self, shape: DigitShape) -> bool {
        let half = 1u128 << (shape.log_basis - 1);
        (St::NEGATIVE_ABS <= half && St::POSITIVE < half)
            || self.accepted.is_some_and(|bounds| {
                bounds.preserves_carrier(St::NEGATIVE_ABS, St::POSITIVE)
                    && bounds.negative_abs.is_some_and(|bound| bound <= half)
                    && bounds.positive.is_some_and(|bound| bound < half)
            })
    }

    fn fill_single<O: Copy + Default>(
        &self,
        rings: Range<usize>,
        raw: &mut [[St::Value; D]],
        out: &mut [[O; D]],
        map: impl Fn(St::Value) -> O,
    ) -> Result<(), AkitaError> {
        if rings.start > rings.end
            || rings.end > self.num_rings
            || raw.len() < rings.len()
            || out.len() < rings.len()
        {
            return Err(AkitaError::InvalidInput(
                "signed block range exceeds its source".into(),
            ));
        }
        out.fill([O::default(); D]);
        let start = rings.start * D;
        let live = self.len.saturating_sub(start).min(rings.len() * D);
        let mut scan = self.scan(false);
        if live != 0 {
            self.storage.copy_map(
                start,
                raw.as_flattened_mut(),
                &mut out.as_flattened_mut()[..live],
                &mut scan,
                map,
            );
        }
        self.check(&scan)
    }

    fn load(
        &self,
        rings: Range<usize>,
        raw: &mut [[St::Value; D]],
        scan: &mut Option<Scan>,
    ) -> Result<(), AkitaError> {
        if rings.start > rings.end || rings.end > self.num_rings || raw.len() < rings.len() {
            return Err(AkitaError::InvalidInput(
                "signed block range exceeds its source".into(),
            ));
        }
        raw.fill([St::Value::default(); D]);
        let start = rings.start * D;
        let live = self.len.saturating_sub(start).min(rings.len() * D);
        if live != 0 {
            self.storage
                .load(start, &mut raw.as_flattened_mut()[..live], scan);
        }
        self.check(scan)
    }
}

fn scanned_digits_fit(scan: &Option<Scan>, shape: DigitShape) -> bool {
    let half = 1i64 << (shape.log_basis - 1);
    scan.as_ref()
        .is_none_or(|scan| scan.min >= -half && scan.max < half)
}

pub(crate) struct SignedScratch<F: Field + CanonicalEncoding, V, const D: usize> {
    params: BalancedDecomposePow2Params<F>,
    raw: Vec<[V; D]>,
    i8: Vec<[i8; D]>,
    i16: Vec<[i16; D]>,
}

impl<F: Field + CanonicalEncoding, V: Copy + Default, const D: usize> SignedScratch<F, V, D> {
    fn new(shape: DigitShape, max_rings: usize) -> Self {
        let width = max_rings * shape.num_digits;
        Self {
            params: BalancedDecomposePow2Params::new(shape.num_digits, shape.log_basis),
            raw: vec![[V::default(); D]; max_rings],
            i8: vec![[0; D]; width],
            i16: vec![[0; D]; width],
        }
    }
}

impl<F, St, const D: usize> RingValueSource<F, D> for SignedBlockDigits<'_, F, St, D>
where
    F: Field,
    St: SignedStorage,
    [St::Value; D]: RingValues<F, D>,
{
    type Values<'a>
        = [St::Value; D]
    where
        Self: 'a;
    fn num_rings(&self) -> usize {
        self.num_rings
    }
    fn values(&self, ring: usize) -> Result<Self::Values<'_>, AkitaError> {
        if ring >= self.num_rings {
            return Err(AkitaError::InvalidInput(
                "ring value index is outside the source".into(),
            ));
        }
        let mut values = [St::Value::default(); D];
        let start = ring * D;
        let live = self.len.saturating_sub(start).min(D);
        if live != 0 {
            self.storage.load(start, &mut values[..live], &mut None);
        }
        Ok(values)
    }
}

impl<F, St, const D: usize> BlockDigits<D> for SignedBlockDigits<'_, F, St, D>
where
    F: Field + CanonicalEncoding,
    St: SignedStorage,
    [St::Value; D]: RingValues<F, D>,
{
    type Scratch = SignedScratch<F, St::Value, D>;
    type Planes<'a>
        = DigitPlanes<'a, D>
    where
        Self: 'a;
    fn num_rings(&self) -> usize {
        self.num_rings
    }
    fn scratch(&self, shape: DigitShape, max_rings: usize) -> Self::Scratch {
        SignedScratch::new(shape, max_rings)
    }
    fn digit_abs_bound(&self, shape: DigitShape) -> u64 {
        1 << (shape.log_basis - 1)
    }
    fn i8_digit_span(&self, shape: DigitShape) -> Option<I8DigitSpan> {
        matches!(
            SignedDigitKernel::for_log_basis(shape.log_basis),
            Some(SignedDigitKernel::I8)
        )
        .then_some(if shape.log_basis <= 7 {
            I8DigitSpan::Half
        } else {
            I8DigitSpan::Full
        })
    }
    fn digit_planes<'a>(
        &'a self,
        ring: usize,
        shape: DigitShape,
        s: &'a mut Self::Scratch,
    ) -> Result<Self::Planes<'a>, AkitaError> {
        match SignedDigitBasis::new(shape.log_basis)?.kernel {
            SignedDigitKernel::I8 => Ok(DigitPlanes::I8(self.rows_i8(ring..ring + 1, shape, s)?)),
            SignedDigitKernel::I16 => Ok(DigitPlanes::I16(
                &self.rows_i16(ring..ring + 1, shape, s)?[..shape.num_digits],
            )),
        }
    }
}

impl<F, St, const D: usize> SignedBlockDigits<'_, F, St, D>
where
    F: Field + CanonicalEncoding,
    St: SignedStorage,
    [St::Value; D]: RingValues<F, D>,
{
    fn rows_i8<'a>(
        &'a self,
        rings: Range<usize>,
        shape: DigitShape,
        s: &'a mut SignedScratch<F, St::Value, D>,
    ) -> Result<&'a [[i8; D]], AkitaError> {
        let proven_fit = self.single_fits(shape);
        let mut scan = self.scan(shape.num_digits == 1 && !proven_fit);
        if shape.num_digits == 1 && rings.start <= rings.end && rings.end <= self.len / D {
            if let Some(rows) = self.storage.borrowed_i8::<D>(rings.clone()) {
                if scan.is_some() {
                    for &value in rows.as_flattened() {
                        track(&mut scan, i64::from(value));
                    }
                }
                self.check(&scan)?;
                if proven_fit || scanned_digits_fit(&scan, shape) {
                    return Ok(rows);
                }
            }
        }
        if shape.num_digits == 1 && proven_fit {
            self.fill_single(rings.clone(), &mut s.raw, &mut s.i8, |value| {
                value.into() as i8
            })?;
            return Ok(&s.i8[..rings.len()]);
        }
        self.load(rings.clone(), &mut s.raw, &mut scan)?;
        let width = rings.len() * shape.num_digits;
        if shape.num_digits == 1 && (proven_fit || scanned_digits_fit(&scan, shape)) {
            for (dst, raw) in s.i8[..width].iter_mut().zip(&s.raw) {
                *dst = raw.map(|value| value.into() as i8);
            }
        } else {
            for (raw, dst) in s.raw[..rings.len()]
                .iter()
                .zip(s.i8[..width].chunks_mut(shape.num_digits))
            {
                s.params.decompose_small_signed_into(raw, dst);
            }
        }
        Ok(&s.i8[..width])
    }
    fn rows_i16<'a>(
        &'a self,
        rings: Range<usize>,
        shape: DigitShape,
        s: &'a mut SignedScratch<F, St::Value, D>,
    ) -> Result<&'a [[i16; D]], AkitaError> {
        let proven_fit = self.single_fits(shape);
        if shape.num_digits == 1 && proven_fit {
            self.fill_single(rings, &mut s.raw, &mut s.i16, |value| value.into() as i16)?;
            return Ok(&s.i16);
        }
        let mut scan = self.scan(shape.num_digits == 1 && !proven_fit);
        self.load(rings.clone(), &mut s.raw, &mut scan)?;
        s.i16.fill([0; D]);
        if shape.num_digits == 1 && (proven_fit || scanned_digits_fit(&scan, shape)) {
            for (dst, raw) in s.i16.iter_mut().zip(&s.raw) {
                *dst = raw.map(|value| value.into() as i16);
            }
        } else {
            for (raw, dst) in s.raw[..rings.len()]
                .iter()
                .zip(s.i16.chunks_mut(shape.num_digits))
            {
                s.params.decompose_small_signed_into(raw, dst);
            }
        }
        Ok(&s.i16)
    }
}

impl<F, St, const D: usize> I8Rows<D> for SignedBlockDigits<'_, F, St, D>
where
    F: Field + CanonicalEncoding,
    St: SignedStorage,
    [St::Value; D]: RingValues<F, D>,
{
    type I8Scratch = SignedScratch<F, St::Value, D>;
    fn i8_scratch(&self, shape: DigitShape, max_rings: usize) -> Self::I8Scratch {
        SignedScratch::new(shape, max_rings)
    }
    fn digit_abs_bound(&self, shape: DigitShape) -> u64 {
        <Self as BlockDigits<D>>::digit_abs_bound(self, shape)
    }
    fn i8_rows<'a>(
        &'a self,
        _block_index: usize,
        block_start: usize,
        rings: Range<usize>,
        shape: DigitShape,
        s: &'a mut Self::I8Scratch,
    ) -> Result<&'a [[i8; D]], AkitaError> {
        self.rows_i8(block_start + rings.start..block_start + rings.end, shape, s)
    }
}

impl<F, St, const D: usize> I16Rows<D> for SignedBlockDigits<'_, F, St, D>
where
    F: Field + CanonicalEncoding,
    St: SignedStorage,
    [St::Value; D]: RingValues<F, D>,
{
    type I16Scratch = SignedScratch<F, St::Value, D>;
    fn i16_scratch(&self, shape: DigitShape, max_rings: usize) -> Self::I16Scratch {
        SignedScratch::new(shape, max_rings)
    }
    fn i16_rows<'a>(
        &'a self,
        rings: Range<usize>,
        shape: DigitShape,
        s: &'a mut Self::I16Scratch,
    ) -> Result<&'a [[i16; D]], AkitaError> {
        self.rows_i16(rings, shape, s)
    }
}
