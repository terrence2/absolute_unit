use crate::{
    impl_value_type_conversions, supports_absdiffeq, supports_cancellation, supports_quantity_ops,
    supports_scalar_ops, supports_shift_ops, supports_value_type_conversion, Acceleration, Length,
    LengthUnit, Scalar, Unit, Velocity, V3,
};
use ordered_float::OrderedFloat;
use std::{fmt, fmt::Debug, marker::PhantomData, ops::Mul};

pub trait TimeUnit: Unit + Copy + Debug + Eq + PartialEq + 'static {
    const SECONDS_IN_UNIT: f64;
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Time<Unit: TimeUnit> {
    v: OrderedFloat<f64>, // in Unit
    phantom_1: PhantomData<Unit>,
}
supports_quantity_ops!(Time<A>, TimeUnit);
supports_shift_ops!(Time<A1>, Time<A2>, TimeUnit);
supports_scalar_ops!(Time<A>, TimeUnit);
supports_cancellation!(Time<A1>, Time<A2>, TimeUnit);
supports_absdiffeq!(Time<A>, TimeUnit);
supports_value_type_conversion!(Time<A>, TimeUnit, impl_value_type_conversions);

impl<Unit> fmt::Display for Time<Unit>
where
    Unit: TimeUnit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0.4}{}", self.v, Unit::UNIT_SHORT_NAME)
    }
}

impl<'a, UnitA, UnitB> From<&'a Time<UnitA>> for Time<UnitB>
where
    UnitA: TimeUnit,
    UnitB: TimeUnit,
{
    fn from(v: &'a Time<UnitA>) -> Self {
        Self {
            v: v.v * UnitA::SECONDS_IN_UNIT / UnitB::SECONDS_IN_UNIT,
            phantom_1: PhantomData,
        }
    }
}

impl<UnitT0, UnitL, UnitT1> Mul<V3<Velocity<UnitL, UnitT1>>> for Time<UnitT0>
where
    UnitT0: TimeUnit,
    UnitL: LengthUnit,
    UnitT1: TimeUnit,
{
    type Output = V3<Length<UnitL>>;

    fn mul(self, rhs: V3<Velocity<UnitL, UnitT1>>) -> Self::Output {
        V3::new_dvec3(*rhs.dvec3() * Time::<UnitT1>::from(&self).f64())
    }
}

impl<UnitT0, UnitL, UnitT1> Mul<Time<UnitT0>> for V3<Velocity<UnitL, UnitT1>>
where
    UnitT0: TimeUnit,
    UnitL: LengthUnit,
    UnitT1: TimeUnit,
{
    type Output = V3<Length<UnitL>>;

    fn mul(self, rhs: Time<UnitT0>) -> Self::Output {
        V3::new_dvec3(*self.dvec3() * Time::<UnitT1>::from(&rhs).f64())
    }
}

impl<UnitT0, UnitL, UnitT1> Mul<V3<Acceleration<UnitL, UnitT1>>> for Time<UnitT0>
where
    UnitT0: TimeUnit,
    UnitL: LengthUnit,
    UnitT1: TimeUnit,
{
    type Output = V3<Velocity<UnitL, UnitT1>>;

    fn mul(self, rhs: V3<Acceleration<UnitL, UnitT1>>) -> Self::Output {
        V3::new_dvec3(*rhs.dvec3() * Time::<UnitT1>::from(&self).f64())
    }
}

impl<UnitL, UnitT0, UnitT1> Mul<Time<UnitT0>> for V3<Acceleration<UnitL, UnitT1>>
where
    UnitL: LengthUnit,
    UnitT0: TimeUnit,
    UnitT1: TimeUnit,
{
    type Output = V3<Velocity<UnitL, UnitT1>>;

    fn mul(self, other: Time<UnitT0>) -> Self::Output {
        V3::<Velocity<UnitL, UnitT1>>::new_dvec3(self.dvec3() * Time::<UnitT1>::from(&other).f64())
    }
}

#[cfg(test)]
mod test {
    use crate::{hours, scalar, seconds};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_time() {
        let h = hours!(1);
        println!("h: {h}");
        println!("s: {}", seconds!(h));
        assert_abs_diff_eq!(seconds!(h), seconds!(3_600));
    }

    #[test]
    fn test_time_scalar() {
        assert_abs_diff_eq!(seconds!(2) * scalar!(2), seconds!(4));
    }
}
