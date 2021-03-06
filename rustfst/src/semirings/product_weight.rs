use std::fmt;
use std::fmt::Debug;

use failure::Fallible;

use crate::semirings::{
    DivideType, Semiring, SemiringProperties, WeaklyDivisibleSemiring, WeightQuantize,
};

/// Product semiring: W1 * W2.
#[derive(Debug, Eq, PartialOrd, PartialEq, Clone, Default, Hash)]
pub struct ProductWeight<W1, W2>
where
    W1: Semiring,
    W2: Semiring,
{
    pub(crate) weight: (W1, W2),
}

impl<W1, W2> fmt::Display for ProductWeight<W1, W2>
where
    W1: Semiring,
    W2: Semiring,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (&self.value1(), &self.value2()).fmt(f)
    }
}

impl<W1, W2> AsRef<Self> for ProductWeight<W1, W2>
where
    W1: Semiring,
    W2: Semiring,
{
    fn as_ref(&self) -> &ProductWeight<W1, W2> {
        &self
    }
}

impl<W1, W2> Semiring for ProductWeight<W1, W2>
where
    W1: Semiring,
    W2: Semiring,
{
    type Type = (W1, W2);
    type ReverseWeight = ProductWeight<W1::ReverseWeight, W2::ReverseWeight>;

    fn zero() -> Self {
        Self {
            weight: (W1::zero(), W2::zero()),
        }
    }

    fn one() -> Self {
        Self {
            weight: (W1::one(), W2::one()),
        }
    }

    fn new(weight: <Self as Semiring>::Type) -> Self {
        Self { weight }
    }

    fn plus_assign<P: AsRef<Self>>(&mut self, rhs: P) -> Fallible<()> {
        self.weight.0.plus_assign(&rhs.as_ref().weight.0)?;
        self.weight.1.plus_assign(&rhs.as_ref().weight.1)?;
        Ok(())
    }

    fn times_assign<P: AsRef<Self>>(&mut self, rhs: P) -> Fallible<()> {
        self.weight.0.times_assign(&rhs.as_ref().weight.0)?;
        self.weight.1.times_assign(&rhs.as_ref().weight.1)?;
        Ok(())
    }

    fn value(&self) -> &<Self as Semiring>::Type {
        &self.weight
    }

    fn take_value(self) -> <Self as Semiring>::Type {
        self.weight
    }

    fn set_value(&mut self, value: <Self as Semiring>::Type) {
        self.set_value1(value.0);
        self.set_value2(value.1);
    }

    fn reverse(&self) -> Fallible<Self::ReverseWeight> {
        Ok((self.value1().reverse()?, self.value2().reverse()?).into())
    }

    fn properties() -> SemiringProperties {
        W1::properties()
            & W2::properties()
            & (SemiringProperties::LEFT_SEMIRING
                | SemiringProperties::RIGHT_SEMIRING
                | SemiringProperties::COMMUTATIVE
                | SemiringProperties::IDEMPOTENT)
    }
}

impl<W1, W2> ProductWeight<W1, W2>
where
    W1: Semiring,
    W2: Semiring,
{
    pub fn value1(&self) -> &W1 {
        &self.weight.0
    }

    pub fn value2(&self) -> &W2 {
        &self.weight.1
    }

    pub fn set_value1(&mut self, new_weight: W1) {
        self.weight.0 = new_weight;
    }

    pub fn set_value2(&mut self, new_weight: W2) {
        self.weight.1 = new_weight;
    }
}

impl<W1, W2> From<(W1, W2)> for ProductWeight<W1, W2>
where
    W1: Semiring,
    W2: Semiring,
{
    fn from(t: (W1, W2)) -> Self {
        Self::new(t)
    }
}

impl<W1, W2> WeaklyDivisibleSemiring for ProductWeight<W1, W2>
where
    W1: WeaklyDivisibleSemiring,
    W2: WeaklyDivisibleSemiring,
{
    fn divide_assign(&mut self, rhs: &Self, divide_type: DivideType) -> Fallible<()> {
        self.weight.0.divide_assign(&rhs.weight.0, divide_type)?;
        self.weight.1.divide_assign(&rhs.weight.1, divide_type)?;
        Ok(())
    }
}

impl<W1, W2> WeightQuantize for ProductWeight<W1, W2>
where
    W1: WeightQuantize,
    W2: WeightQuantize,
{
    fn quantize_assign(&mut self, delta: f32) -> Fallible<()> {
        self.set_value1(self.value1().quantize(delta)?);
        self.set_value2(self.value2().quantize(delta)?);
        Ok(())
    }
}
