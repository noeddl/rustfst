use std::fmt;

use failure::Fallible;

use crate::semirings::string_variant::StringWeightVariant;
use crate::semirings::{
    DivideType, Semiring, SemiringProperties, WeaklyDivisibleSemiring, WeightQuantize,
};
use crate::Label;

/// String semiring: (identity, ., Infinity, Epsilon)
#[derive(Clone, Debug, PartialOrd, Default, PartialEq, Eq, Hash)]
pub struct StringWeightRestrict {
    pub(crate) value: StringWeightVariant,
}

/// String semiring: (longest_common_prefix, ., Infinity, Epsilon)
#[derive(Clone, Debug, PartialOrd, Default, PartialEq, Eq, Hash)]
pub struct StringWeightLeft {
    pub(crate) value: StringWeightVariant,
}

/// String semiring: (longest_common_suffix, ., Infinity, Epsilon)
#[derive(Clone, Debug, PartialOrd, Default, PartialEq, Eq, Hash)]
pub struct StringWeightRight {
    pub(crate) value: StringWeightVariant,
}

/// Determines whether to use left or right string semiring.  Includes a
/// 'restricted' version that signals an error if proper prefixes/suffixes
/// would otherwise be returned by Plus, useful with various
/// algorithms that require functional transducer input with the
/// string semirings.
pub enum StringType {
    StringRestrict,
    StringLeft,
    StringRight,
}

macro_rules! string_semiring {
    ($semiring: ty, $string_type: expr, $reverse_semiring: ty) => {
        impl fmt::Display for $semiring {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match &self.value {
                    StringWeightVariant::Infinity => {
                        write!(f, "Infinity")?;
                    }
                    StringWeightVariant::Labels(v) => {
                        if v.is_empty() {
                            write!(f, "Epsilon")?;
                        } else {
                            // FIXME
                            write!(f, "{:?}", v)?;
                        }
                    }
                };
                Ok(())
            }
        }

        impl AsRef<Self> for $semiring {
            fn as_ref(&self) -> &$semiring {
                &self
            }
        }

        impl Semiring for $semiring {
            type Type = StringWeightVariant;
            type ReverseWeight = $reverse_semiring;

            fn zero() -> Self {
                Self {
                    value: StringWeightVariant::Infinity,
                }
            }

            fn one() -> Self {
                Self {
                    value: StringWeightVariant::Labels(vec![]),
                }
            }

            fn new(value: <Self as Semiring>::Type) -> Self {
                Self { value }
            }

            fn plus_assign<P: AsRef<Self>>(&mut self, rhs: P) -> Fallible<()> {
                if self.is_zero() {
                    self.set_value(rhs.as_ref().value().clone());
                } else if rhs.as_ref().is_zero() {
                    // Do nothing
                } else {
                    let l1 = self.value.unwrap_labels();
                    let l2 = rhs.as_ref().value.unwrap_labels();

                    match $string_type {
                        StringType::StringRestrict => {
                            if self != rhs.as_ref() {
                                bail!(
                                    "Unequal arguments : non-functional FST ? w1 = {:?} w2 = {:?}",
                                    &self,
                                    &rhs.as_ref()
                                );
                            }
                        }
                        StringType::StringLeft => {
                            let new_labels: Vec<_> = l1
                                .iter()
                                .zip(l2.iter())
                                .take_while(|(v1, v2)| v1 == v2)
                                .map(|(v1, _)| v1)
                                .cloned()
                                .collect();
                            self.value = StringWeightVariant::Labels(new_labels);
                        }
                        StringType::StringRight => {
                            let new_labels: Vec<_> = l1
                                .iter()
                                .rev()
                                .zip(l2.iter().rev())
                                .take_while(|(v1, v2)| v1 == v2)
                                .map(|(v1, _)| v1)
                                .cloned()
                                .collect();
                            let new_labels: Vec<_> = new_labels.into_iter().rev().collect();
                            self.value = StringWeightVariant::Labels(new_labels);
                        }
                    };
                };
                Ok(())
            }
            fn times_assign<P: AsRef<Self>>(&mut self, rhs: P) -> Fallible<()> {
                if let StringWeightVariant::Labels(ref mut labels_left) = self.value {
                    if let StringWeightVariant::Labels(ref labels_right) = rhs.as_ref().value {
                        for l in labels_right {
                            labels_left.push(*l);
                        }
                    } else {
                        self.value = StringWeightVariant::Infinity;
                    }
                }
                Ok(())
            }

            fn value(&self) -> &<Self as Semiring>::Type {
                &self.value
            }

            fn take_value(self) -> <Self as Semiring>::Type {
                self.value
            }

            fn set_value(&mut self, value: <Self as Semiring>::Type) {
                self.value = value;
            }

            fn reverse(&self) -> Fallible<Self::ReverseWeight> {
                Ok(self.value().reverse().into())
            }

            fn properties() -> SemiringProperties {
                match $string_type {
                    StringType::StringRestrict => {
                        SemiringProperties::LEFT_SEMIRING
                            | SemiringProperties::RIGHT_SEMIRING
                            | SemiringProperties::IDEMPOTENT
                    }
                    StringType::StringLeft => {
                        SemiringProperties::LEFT_SEMIRING | SemiringProperties::IDEMPOTENT
                    }
                    StringType::StringRight => {
                        SemiringProperties::RIGHT_SEMIRING | SemiringProperties::IDEMPOTENT
                    }
                }
            }
        }

        impl $semiring {
            pub fn len_labels(&self) -> usize {
                match &self.value {
                    StringWeightVariant::Infinity => 0,
                    StringWeightVariant::Labels(l) => l.len(),
                }
            }

            pub fn iter(&self) -> impl Iterator<Item = StringWeightVariant> + '_ {
                self.value.iter()
            }
        }

        impl From<Vec<Label>> for $semiring {
            fn from(l: Vec<usize>) -> Self {
                Self::new(l.into())
            }
        }

        impl From<Label> for $semiring {
            fn from(l: usize) -> Self {
                Self::new(vec![l].into())
            }
        }

        impl From<StringWeightVariant> for $semiring {
            fn from(v: StringWeightVariant) -> Self {
                Self::new(v)
            }
        }

        impl WeightQuantize for $semiring {
            fn quantize_assign(&mut self, _delta: f32) -> Fallible<()> {
                // Nothing to do
                Ok(())
            }
        }
    };
}

string_semiring!(
    StringWeightRestrict,
    StringType::StringRestrict,
    StringWeightRestrict
);
string_semiring!(StringWeightLeft, StringType::StringLeft, StringWeightRight);
string_semiring!(StringWeightRight, StringType::StringRight, StringWeightLeft);

fn divide_left(w1: &StringWeightVariant, w2: &StringWeightVariant) -> StringWeightVariant {
    match (w1, w2) {
        (StringWeightVariant::Infinity, StringWeightVariant::Infinity) => panic!("Unexpected"),
        (StringWeightVariant::Infinity, StringWeightVariant::Labels(_)) => {
            StringWeightVariant::Infinity
        }
        (StringWeightVariant::Labels(_), StringWeightVariant::Infinity) => panic!("Unexpected"),
        (StringWeightVariant::Labels(l1), StringWeightVariant::Labels(l2)) => {
            StringWeightVariant::Labels(l1.iter().skip(l2.len()).cloned().collect())
        }
    }
}

fn divide_right(w1: &StringWeightVariant, w2: &StringWeightVariant) -> StringWeightVariant {
    match (w1, w2) {
        (StringWeightVariant::Infinity, StringWeightVariant::Infinity) => panic!("Unexpected"),
        (StringWeightVariant::Infinity, StringWeightVariant::Labels(_)) => {
            StringWeightVariant::Infinity
        }
        (StringWeightVariant::Labels(_), StringWeightVariant::Infinity) => panic!("Unexpected"),
        (StringWeightVariant::Labels(l1), StringWeightVariant::Labels(l2)) => {
            StringWeightVariant::Labels(l1.iter().rev().skip(l2.len()).rev().cloned().collect())
        }
    }
}

impl WeaklyDivisibleSemiring for StringWeightLeft {
    fn divide_assign(&mut self, rhs: &Self, divide_type: DivideType) -> Fallible<()> {
        if divide_type != DivideType::DivideLeft {
            bail!("Only left division is defined.");
        }
        self.value = divide_left(&self.value, &rhs.value);
        Ok(())
    }
}

impl WeaklyDivisibleSemiring for StringWeightRight {
    fn divide_assign(&mut self, rhs: &Self, divide_type: DivideType) -> Fallible<()> {
        if divide_type != DivideType::DivideRight {
            bail!("Only right division is defined.");
        }
        self.value = divide_right(&self.value, &rhs.value);
        Ok(())
    }
}

impl WeaklyDivisibleSemiring for StringWeightRestrict {
    fn divide_assign(&mut self, rhs: &Self, divide_type: DivideType) -> Fallible<()> {
        self.value = match divide_type {
            DivideType::DivideLeft => divide_left(&self.value, &rhs.value),
            DivideType::DivideRight => divide_right(&self.value, &rhs.value),
            DivideType::DivideAny => bail!("Only explicit left or right division is defined."),
        };
        Ok(())
    }
}
