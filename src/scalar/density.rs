use crate::{
    impl_value_type_conversions, supports_absdiffeq, supports_cancellation, supports_quantity_ops,
    supports_scalar_ops, supports_shift_ops, supports_value_type_conversion, DynamicUnits,
    LengthUnit, MassUnit,
};
use ordered_float::OrderedFloat;
use std::{fmt, fmt::Debug, marker::PhantomData};

// mass / length^3
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Density<UnitMass: MassUnit, UnitLength: LengthUnit> {
    v: OrderedFloat<f64>,
    phantom_1: PhantomData<UnitMass>,
    phantom_2: PhantomData<UnitLength>,
}
supports_absdiffeq!(Density<A, B>, MassUnit, LengthUnit);
supports_quantity_ops!(Density<A, B>, MassUnit, LengthUnit);
supports_scalar_ops!(Density<A, B>, MassUnit, LengthUnit);
supports_cancellation!(Density<A1, B1>, Density<A2, B2>, MassUnit, LengthUnit);
supports_shift_ops!(Density<A1, B1>, Density<A2, B2>, MassUnit, LengthUnit);
supports_value_type_conversion!(Density<A, B>, MassUnit, LengthUnit, impl_value_type_conversions);

impl<M, L> fmt::Display for Density<M, L>
where
    M: MassUnit,
    L: LengthUnit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.v.0, f)?;
        write!(f, "{}/{}^3", M::UNIT_SHORT_NAME, L::UNIT_SHORT_NAME)
    }
}

impl<'a, MA, LA, MB, LB> From<&'a Density<MB, LB>> for Density<MA, LA>
where
    MA: MassUnit,
    LA: LengthUnit,
    MB: MassUnit,
    LB: LengthUnit,
{
    fn from(v: &'a Density<MB, LB>) -> Self {
        let mass_ratio = MB::GRAMS_IN_UNIT / MA::GRAMS_IN_UNIT;
        let length_ratio = LA::METERS_IN_UNIT / LB::METERS_IN_UNIT;
        Self {
            v: v.v * mass_ratio * length_ratio * length_ratio * length_ratio,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }
}

impl<MA, LA> Density<MA, LA>
where
    MA: MassUnit,
    LA: LengthUnit,
{
    pub fn as_dyn(&self) -> DynamicUnits {
        DynamicUnits::new1o3::<MA, LA, LA, LA>(self.v)
    }
}

#[cfg(test)]
mod test {
    use crate::{kilograms_per_meter3, slugs_per_foot3};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_density() {
        let s_p_f3 = slugs_per_foot3!(100f64);
        let kg_p_m3 = kilograms_per_meter3!(s_p_f3);
        println!("{s_p_f3}");
        println!("{kg_p_m3}");
        assert_abs_diff_eq!(s_p_f3, slugs_per_foot3!(kg_p_m3), epsilon = 0.000_000_1);
    }
}
