use std::fmt::{Display, Write};

use num_rational::Ratio;
use num_traits::{ToPrimitive, Zero};

use crate::{
    arithmetic::{Exponent, Power, Rational},
    number::Number,
    prefix::Prefix,
    product::{Canonicalize, Product},
};

pub type ConversionFactor = Number;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitType {
    Standard,
    NonStandard(ConversionFactor, Unit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseUnit {
    name: String,
    unit_type: UnitType,
}

impl BaseUnit {
    fn to_standard(&self) -> Unit {
        match &self.unit_type {
            UnitType::Standard => Unit::new_standard(&self.name),
            UnitType::NonStandard(_, unit) => unit.clone(),
        }
    }

    fn conversion_factor(&self) -> Number {
        match &self.unit_type {
            UnitType::Standard => Number::from_f64(1.0),
            UnitType::NonStandard(factor, _) => factor.clone(),
        }
    }
}

impl PartialOrd for BaseUnit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for BaseUnit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct UnitFactor(pub Prefix, pub BaseUnit, pub Exponent);

impl Canonicalize for UnitFactor {
    type MergeKey = BaseUnit;

    fn merge_key(&self) -> Self::MergeKey {
        self.1.clone()
    }

    fn merge(self, other: Self) -> Self {
        UnitFactor(self.0, self.1, self.2 + other.2)
    }

    fn is_trivial(&self) -> bool {
        self.2 == Rational::zero()
    }
}

impl Power for UnitFactor {
    fn power(self, e: Exponent) -> Self {
        UnitFactor(self.0, self.1, self.2 * e)
    }
}

pub type Unit = Product<UnitFactor, false>;

impl Unit {
    pub fn scalar() -> Self {
        Self::unity()
    }

    pub fn new_standard(name: &str) -> Self {
        Unit::from_factor(UnitFactor(
            Prefix::none(),
            BaseUnit {
                name: name.into(),
                unit_type: UnitType::Standard,
            },
            Rational::from_integer(1),
        ))
    }

    pub fn new_non_standard(name: &str, factor: ConversionFactor, standard_unit: Unit) -> Self {
        Unit::from_factor(UnitFactor(
            Prefix::none(),
            BaseUnit {
                name: name.into(),
                unit_type: UnitType::NonStandard(factor, standard_unit),
            },
            Rational::from_integer(1),
        ))
    }

    pub fn with_prefix(self, prefix: Prefix) -> Self {
        let mut factors: Vec<_> = self.into_iter().collect();
        assert!(!factors.is_empty());
        assert!(factors[0].0 == Prefix::none());
        factors[0].0 = prefix;
        Self::from_factors(factors)
    }

    pub fn to_standard_representation(&self) -> (Self, ConversionFactor) {
        let standardized_unit = self
            .iter()
            .map(|UnitFactor(_, base_unit, exponent)| base_unit.to_standard().power(*exponent))
            .product();

        let factor = self
            .iter()
            .map(|UnitFactor(prefix, base_unit, exponent)| {
                (prefix.factor() * base_unit.conversion_factor())
                    .pow(&Number::from_f64(exponent.to_f64().unwrap()))
            }) // TODO: reduce wrapping/unwrapping; do we want to use exponent.to_f64?
            .product();

        (standardized_unit, factor)
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for &UnitFactor(_prefix, ref base_unit, exp) in self.iter() {
            // TODO: render prefix
            result.push_str(&base_unit.name);

            if exp == Ratio::from_integer(5) {
                result.push('⁵');
            } else if exp == Ratio::from_integer(4) {
                result.push('⁴');
            } else if exp == Ratio::from_integer(3) {
                result.push('³');
            } else if exp == Ratio::from_integer(2) {
                result.push('²');
            } else if exp == Ratio::from_integer(1) {
            } else if exp == Ratio::from_integer(-1) {
                result.push('⁻');
                result.push('¹');
            } else if exp == Ratio::from_integer(-2) {
                result.push('⁻');
                result.push('²');
            } else if exp == Ratio::from_integer(-3) {
                result.push('⁻');
                result.push('³');
            } else if exp == Ratio::from_integer(-4) {
                result.push('⁻');
                result.push('⁴');
            } else if exp == Ratio::from_integer(-5) {
                result.push('⁻');
                result.push('⁵');
            } else {
                write!(result, "^{}", exp).unwrap();
            };
            result.push('·');
        }

        write!(f, "{}", result.trim_end_matches('·'))
    }
}

#[test]
fn unit_basic() {
    let meter = Unit::from_factor(UnitFactor(
        Prefix::none(),
        BaseUnit {
            name: "meter".into(),
            unit_type: UnitType::Standard,
        },
        Rational::from_integer(1),
    ));
    let second = Unit::from_factor(UnitFactor(
        Prefix::none(),
        BaseUnit {
            name: "second".into(),
            unit_type: UnitType::Standard,
        },
        Rational::from_integer(1),
    ));

    let meter_per_second = Unit::from_factors([
        UnitFactor(
            Prefix::none(),
            BaseUnit {
                name: "meter".into(),
                unit_type: UnitType::Standard,
            },
            Rational::from_integer(1),
        ),
        UnitFactor(
            Prefix::none(),
            BaseUnit {
                name: "second".into(),
                unit_type: UnitType::Standard,
            },
            Rational::from_integer(-1),
        ),
    ]);

    assert_eq!(meter.divide(second), meter_per_second);
}
