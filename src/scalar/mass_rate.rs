use crate::{
    impl_value_type_conversions, supports_absdiffeq, supports_cancellation, supports_quantity_ops,
    supports_scalar_ops, supports_shift_ops, supports_value_type_conversion, DynamicUnits, Mass,
    MassUnit, Time, TimeUnit,
};
use ordered_float::OrderedFloat;
use std::{fmt, fmt::Debug, marker::PhantomData, ops::Mul};

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct MassRate<UnitMass: MassUnit, UnitTime: TimeUnit> {
    v: OrderedFloat<f64>,
    phantom_1: PhantomData<UnitMass>,
    phantom_2: PhantomData<UnitTime>,
}
supports_quantity_ops!(MassRate<A, B>, MassUnit, TimeUnit);
supports_shift_ops!(MassRate<A1, B1>, MassRate<A2, B2>, MassUnit, TimeUnit);
supports_scalar_ops!(MassRate<A, B>, MassUnit, TimeUnit);
supports_cancellation!(MassRate<A1, B1>, MassRate<A2, B2>, MassUnit, TimeUnit);
supports_absdiffeq!(MassRate<A, B>, MassUnit, TimeUnit);
supports_value_type_conversion!(MassRate<A, B>, MassUnit, TimeUnit, impl_value_type_conversions);

impl<M, T> fmt::Display for MassRate<M, T>
where
    M: MassUnit,
    T: TimeUnit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.v.0, f)?;
        write!(f, "{}/{}", M::UNIT_SHORT_NAME, T::UNIT_SHORT_NAME)
    }
}

impl<'a, MA, TA, MB, TB> From<&'a MassRate<MA, TA>> for MassRate<MB, TB>
where
    MA: MassUnit,
    TA: TimeUnit,
    MB: MassUnit,
    TB: TimeUnit,
{
    fn from(v: &'a MassRate<MA, TA>) -> Self {
        let length_ratio = MA::GRAMS_IN_UNIT / MB::GRAMS_IN_UNIT;
        let time_ratio = TB::SECONDS_IN_UNIT / TA::SECONDS_IN_UNIT;
        Self {
            v: v.v * length_ratio * time_ratio,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }
}

impl<MA, TA> MassRate<MA, TA>
where
    MA: MassUnit,
    TA: TimeUnit,
{
    pub fn as_dyn(&self) -> DynamicUnits {
        DynamicUnits::new1o1::<MA, TA>(self.v)
    }
}

impl<MA, TA, TB> Mul<Time<TB>> for MassRate<MA, TA>
where
    MA: MassUnit,
    TA: TimeUnit,
    TB: TimeUnit,
{
    type Output = Mass<MA>;

    fn mul(self, other: Time<TB>) -> Self::Output {
        Mass::<MA>::from(self.v.0 * Time::<TA>::from(&other).f64())
    }
}

#[cfg(test)]
mod test {
    use crate::{kilograms_per_second, pounds_mass_per_second};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_mass_rate() {
        let lbps = pounds_mass_per_second!(100_f64);
        let kgps = kilograms_per_second!(45.359237_f64);
        println!("{lbps} vs {kgps}");
        assert_abs_diff_eq!(kilograms_per_second!(lbps), kgps);
        assert_abs_diff_eq!(pounds_mass_per_second!(kgps), lbps);
    }
}
