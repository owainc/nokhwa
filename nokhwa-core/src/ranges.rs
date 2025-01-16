use core::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{Div, Rem, Sub};
use ordered_float::OrderedFloat;

/// A range type that can be validated.
pub trait ValidatableRange {
    /// Input type to validate.
    type Validation;

    /// Validates the value.
    fn validate(&self, value: &Self::Validation) -> bool;
}

/// Creates a range of values.
///
/// Inclusive by default.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Range<T> where T: RangeItem
{
    minimum: T,
    lower_inclusive: bool,
    maximum: T,
    upper_inclusive: bool,
    step: Option<T>,
}

impl<T> Range<T> where T: Copy {
    /// Create an upper and lower inclusive [`Range`]
    pub fn new(min: T, max: T, step: Option<T>) -> Self {
        Self {
            minimum: min,
            lower_inclusive: true,
            maximum: max,
            upper_inclusive: true,
            step,
        }
    }

    pub fn with_inclusive(
        min: T,
        lower_inclusive: bool,
        max: T,
        upper_inclusive: bool,
        step: Option<T>
    ) -> Self {
        Self {
            minimum: min,
            lower_inclusive,
            maximum: max,
            upper_inclusive,
            step,
        }
    }

    pub fn set_minimum(&mut self, minimum: Option<T>) {
        self.minimum = minimum;
    }
    pub fn set_lower_inclusive(&mut self, lower_inclusive: bool) {
        self.lower_inclusive = lower_inclusive;
    }
    pub fn set_maximum(&mut self, maximum: Option<T>) {
        self.maximum = maximum;
    }
    pub fn set_upper_inclusive(&mut self, upper_inclusive: bool) {
        self.upper_inclusive = upper_inclusive;
    }
    pub fn set_step(&mut self, step: T) {
        self.step = Some(step);
    }
    pub fn minimum(&self) -> T {
        self.minimum
    }
    pub fn lower_inclusive(&self) -> bool {
        self.lower_inclusive
    }
    pub fn maximum(&self) -> T {
        self.maximum
    }
    pub fn upper_inclusive(&self) -> bool {
        self.upper_inclusive
    }
    pub fn step(&self) -> Option<T> {
        self.step
    }
}

impl<T> ValidatableRange for Range<T>
where
    T: RangeItem,
{
    type Validation = T;

    fn validate(&self, value: &T) -> bool {
        let l_comparison_fn = match self.lower_inclusive {
            true => T::ge,
            false => T::gt,
        };
        let u_comparison_fn = match self.upper_inclusive {
            true => T::le,
            false => T::lt,
        };

        if !(l_comparison_fn(&self.minimum, value) && u_comparison_fn(&self.maximum, value)) {
            return false
        }

        // check step

        if let Some(step) = self.step {
            let step_chk_value = *value - self.minimum;
            return step_chk_value % step == 0;
        }

        return true
    }
}

impl<T> Default for Range<T>
where
    T: Default,
{
    fn default() -> Self {
        Range {
            minimum: T::default(),
            lower_inclusive: true,
            maximum: T::default(),
            upper_inclusive: true,
            step: None,
        }
    }
}

impl<T> Display for Range<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lower_inclusive_char = bool_to_inclusive_char(self.lower_inclusive, false);
        let upper_inclusive_char = bool_to_inclusive_char(self.upper_inclusive, true);

        write!(
            f,
            "Range: {lower_inclusive_char}{:?}, {:?}{upper_inclusive_char}",
            self.minimum, self.maximum
        )
    }
}

fn bool_to_inclusive_char(inclusive: bool, upper: bool) -> char {
    match inclusive {
        true => {
            if upper {
                ']'
            } else {
                '['
            }
        }
        false => {
            if upper {
                ')'
            } else {
                '('
            }
        }
    }
}

fn default_to_string<T>(default: &Option<T>) -> String
where
    T: Debug,
{
    match default {
        Some(v) => {
            format!("{v:?}")
        }
        None => String::from("None"),
    }
}

pub trait RangeItem: Copy + Clone + Debug + Div<Output = Self> + Sub<Output = Self> + Rem<Output = Self> + Hash + Ord + PartialOrd + Eq + PartialEq {
    const ZERO: Self;
}

macro_rules! impl_num {
    ($($n:ty)*) => ($(
        impl RangeItem for $n {
            const ZERO: $n = 0;
        }
    )*)
}

impl_num! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 }

impl RangeItem for OrderedFloat<f32> {
    const ZERO: Self = OrderedFloat(0_f32);
}

impl RangeItem for OrderedFloat<f64> {
    const ZERO: Self = OrderedFloat(0_f64);
}