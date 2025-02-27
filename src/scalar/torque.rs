use crate::{
    impl_value_type_conversions, supports_absdiffeq, supports_cancellation, supports_quantity_ops,
    supports_scalar_ops, supports_shift_ops, supports_value_type_conversion, DynamicUnits,
    ForceUnit, LengthUnit,
};
use ordered_float::OrderedFloat;
use std::{fmt, fmt::Debug, marker::PhantomData};

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Torque<UnitForce: ForceUnit, UnitLength: LengthUnit> {
    v: OrderedFloat<f64>,
    phantom_1: PhantomData<UnitForce>,
    phantom_2: PhantomData<UnitLength>,
}
supports_quantity_ops!(Torque<A, B>, ForceUnit, LengthUnit);
supports_shift_ops!(Torque<A1, B1>, Torque<A2, B2>, ForceUnit, LengthUnit);
supports_scalar_ops!(Torque<A, B>, ForceUnit, LengthUnit);
supports_cancellation!(Torque<A1, B1>, Torque<A2, B2>, ForceUnit, LengthUnit);
supports_absdiffeq!(Torque<A, B>, ForceUnit, LengthUnit);
supports_value_type_conversion!(Torque<A, B>, ForceUnit, LengthUnit, impl_value_type_conversions);

impl<F, L> fmt::Display for Torque<F, L>
where
    F: ForceUnit,
    L: LengthUnit,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.v.0, f)?;
        write!(f, "{}*{}", F::UNIT_SHORT_NAME, L::UNIT_SHORT_NAME)
    }
}

impl<'a, FA, LA, FB, LB> From<&'a Torque<FA, LA>> for Torque<FB, LB>
where
    FA: ForceUnit,
    LA: LengthUnit,
    FB: ForceUnit,
    LB: LengthUnit,
{
    fn from(v: &'a Torque<FA, LA>) -> Self {
        let force_ratio = FA::NEWTONS_IN_UNIT / FB::NEWTONS_IN_UNIT;
        let length_ratio = LA::METERS_IN_UNIT / LB::METERS_IN_UNIT;
        Self {
            v: v.v * force_ratio * length_ratio,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }
}

impl<F, L> Torque<F, L>
where
    F: ForceUnit,
    L: LengthUnit,
{
    pub fn as_dyn(&self) -> DynamicUnits {
        DynamicUnits::new3o2::<F::UnitMass, F::UnitLength, L, F::UnitTime, F::UnitTime>(self.v)
    }
}

impl<F, L> From<DynamicUnits> for Torque<F, L>
where
    F: ForceUnit,
    L: LengthUnit,
{
    fn from(v: DynamicUnits) -> Self {
        let f = v.ordered_float();
        v.assert_units_equal(DynamicUnits::new3o2::<
            F::UnitMass,
            F::UnitLength,
            L,
            F::UnitTime,
            F::UnitTime,
        >(0f64.into()));
        Self {
            v: f,
            phantom_1: PhantomData,
            phantom_2: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::newton_meters;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_torque() {
        let nm = newton_meters!(100.);
        println!("{nm}");
        assert_abs_diff_eq!(nm, newton_meters!(100.));
    }
}
