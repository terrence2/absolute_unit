#[macro_export]
macro_rules! supports_value_type_conversion {
    ($TypeName:ty, $UnitA:path, $UnitB:path, $it:tt) => {
        $it!(f64, $TypeName, $UnitA, $UnitB);
        $it!(f32, $TypeName, $UnitA, $UnitB);
        $it!(isize, $TypeName, $UnitA, $UnitB);
        $it!(i64, $TypeName, $UnitA, $UnitB);
        $it!(i32, $TypeName, $UnitA, $UnitB);
        $it!(i16, $TypeName, $UnitA, $UnitB);
        $it!(i8, $TypeName, $UnitA, $UnitB);
        $it!(u32, $TypeName, $UnitA, $UnitB);
        $it!(u16, $TypeName, $UnitA, $UnitB);
        $it!(u8, $TypeName, $UnitA, $UnitB);

        impl<A, B> $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            pub fn f64(self) -> f64 {
                f64::from(self)
            }

            pub fn of64(self) -> OrderedFloat<f64> {
                OrderedFloat(f64::from(self))
            }

            pub fn f32(self) -> f32 {
                f32::from(self)
            }

            pub fn of32(self) -> OrderedFloat<f32> {
                OrderedFloat(f32::from(self))
            }

            pub fn is_nan(&self) -> bool {
                self.v.0.is_nan()
            }

            pub fn is_infinite(&self) -> bool {
                self.v.0.is_infinite()
            }

            pub fn is_finite(&self) -> bool {
                self.v.0.is_finite()
            }
        }
    };

    ($TypeName:ty, $Unit:path, $it:tt) => {
        $it!(f64, $TypeName, $Unit);
        $it!(f32, $TypeName, $Unit);
        $it!(isize, $TypeName, $Unit);
        $it!(i64, $TypeName, $Unit);
        $it!(i32, $TypeName, $Unit);
        $it!(i16, $TypeName, $Unit);
        $it!(i8, $TypeName, $Unit);
        $it!(u32, $TypeName, $Unit);
        $it!(u16, $TypeName, $Unit);
        $it!(u8, $TypeName, $Unit);

        impl<A> $TypeName
        where
            A: $Unit,
        {
            pub fn f64(self) -> f64 {
                f64::from(self)
            }

            pub fn of64(self) -> OrderedFloat<f64> {
                OrderedFloat(f64::from(self))
            }

            pub fn f32(self) -> f32 {
                f32::from(self)
            }

            pub fn of32(self) -> OrderedFloat<f32> {
                OrderedFloat(f32::from(self))
            }

            pub fn is_nan(&self) -> bool {
                self.v.0.is_nan()
            }

            pub fn is_infinite(&self) -> bool {
                self.v.0.is_infinite()
            }

            pub fn is_finite(&self) -> bool {
                self.v.0.is_finite()
            }
        }
    };

    ($it:tt) => {
        $it!(f64);
        $it!(f32);
        $it!(isize);
        $it!(i64);
        $it!(i32);
        $it!(i16);
        $it!(i8);
        $it!(u32);
        $it!(u16);
        $it!(u8);
    };
}

#[macro_export]
macro_rules! impl_value_type_conversions {
    (f64, $TypeName:ty, $UnitA:path, $UnitB:path) => {
        impl<A, B> From<f64> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: f64) -> Self {
                Self {
                    v: OrderedFloat(v),
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A, B> From<&f64> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: &f64) -> Self {
                Self {
                    v: OrderedFloat(*v),
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A, B> From<$TypeName> for f64
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: $TypeName) -> f64 {
                v.v.0
            }
        }
    };

    (f64, $TypeName:ty, $UnitA:path) => {
        impl<A> From<f64> for $TypeName
        where
            A: $UnitA,
        {
            fn from(v: f64) -> Self {
                Self {
                    v: OrderedFloat(v),
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A> From<&f64> for $TypeName
        where
            A: $UnitA,
        {
            fn from(v: &f64) -> Self {
                Self {
                    v: OrderedFloat(*v),
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A> From<$TypeName> for f64
        where
            A: $UnitA,
        {
            fn from(v: $TypeName) -> f64 {
                v.v.0
            }
        }
    };

    (f32, $TypeName:ty, $UnitA:path, $UnitB:path) => {
        impl<A, B> From<f32> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: f32) -> Self {
                Self {
                    v: OrderedFloat(v as f64),
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A, B> From<&f32> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: &f32) -> Self {
                Self {
                    v: OrderedFloat(*v as f64),
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A, B> From<$TypeName> for f32
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: $TypeName) -> f32 {
                v.v.0 as f32
            }
        }
    };

    (f32, $TypeName:ty, $UnitA:path) => {
        impl<A> From<f32> for $TypeName
        where
            A: $UnitA,
        {
            fn from(v: f32) -> Self {
                Self {
                    v: OrderedFloat(v as f64),
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A> From<&f32> for $TypeName
        where
            A: $UnitA,
        {
            fn from(v: &f32) -> Self {
                Self {
                    v: OrderedFloat(*v as f64),
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A> From<$TypeName> for f32
        where
            A: $UnitA,
        {
            fn from(v: $TypeName) -> f32 {
                v.v.0 as f32
            }
        }
    };

    ($Num:ty, $TypeName:ty, $UnitA:path, $UnitB:path) => {
        impl<A, B> From<$Num> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: $Num) -> Self {
                Self {
                    v: OrderedFloat(v as f64),
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A, B> From<&$Num> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: &$Num) -> Self {
                Self {
                    v: OrderedFloat(*v as f64),
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A, B> From<$TypeName> for $Num
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn from(v: $TypeName) -> $Num {
                v.v.0.round() as $Num
            }
        }
    };

    ($Num:ty, $TypeName:ty, $UnitA:path) => {
        impl<A> From<$Num> for $TypeName
        where
            A: $UnitA,
        {
            fn from(v: $Num) -> Self {
                Self {
                    v: OrderedFloat(v as f64),
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A> From<&$Num> for $TypeName
        where
            A: $UnitA,
        {
            fn from(v: &$Num) -> Self {
                Self {
                    v: OrderedFloat(*v as f64),
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A> From<$TypeName> for $Num
        where
            A: $UnitA,
        {
            fn from(v: $TypeName) -> $Num {
                v.v.0.round() as $Num
            }
        }
    };
}

#[macro_export]
macro_rules! supports_absdiffeq {
    ($TypeName:ty, $UnitA:path, $UnitB:path) => {
        impl<A, B> $crate::approx::AbsDiffEq for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            type Epsilon = f64;

            fn default_epsilon() -> Self::Epsilon {
                f64::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                self.v.0.abs_diff_eq(&other.v.0, epsilon)
            }
        }

        impl<A, B> $crate::approx::RelativeEq for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn default_max_relative() -> Self::Epsilon {
                use $crate::approx::AbsDiffEq;
                f64::default_epsilon()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                self.v.0.relative_eq(&other.v.0, epsilon, max_relative)
            }
        }
    };

    ($TypeName:ty, $UnitA:path) => {
        impl<A> $crate::approx::AbsDiffEq for $TypeName
        where
            A: $UnitA,
        {
            type Epsilon = f64;

            fn default_epsilon() -> Self::Epsilon {
                f64::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                self.v.0.abs_diff_eq(&other.v.0, epsilon)
            }
        }

        impl<A> $crate::approx::RelativeEq for $TypeName
        where
            A: $UnitA,
        {
            fn default_max_relative() -> Self::Epsilon {
                use $crate::approx::AbsDiffEq;
                f64::default_epsilon()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                self.v.0.relative_eq(&other.v.0, epsilon, max_relative)
            }
        }
    };
}

#[macro_export]
macro_rules! supports_scalar_ops {
    ($TypeName:ty, $UnitA:path, $UnitB:path) => {
        impl<A, B> std::ops::Mul<$crate::Scalar> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            type Output = $TypeName;

            fn mul(self, s: $crate::Scalar) -> Self::Output {
                use $crate::Quantity;
                Self {
                    v: self.v * s.f64(),
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A, B> std::ops::Mul<$TypeName> for $crate::Scalar
        where
            A: $UnitA,
            B: $UnitB,
        {
            type Output = $TypeName;

            fn mul(self, other: $TypeName) -> Self::Output {
                <$TypeName>::from(self.0.into_inner() * other.f64())
            }
        }

        impl<A, B> std::ops::MulAssign<$crate::Scalar> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn mul_assign(&mut self, s: $crate::Scalar) {
                use $crate::Quantity;
                self.v *= s.f64();
            }
        }

        impl<A, B> std::ops::Mul<$TypeName> for $crate::glam::DVec3
        where
            A: $UnitA,
            B: $UnitB,
        {
            type Output = $crate::V3<$TypeName>;

            fn mul(self, other: $TypeName) -> Self::Output {
                $crate::V3::<$TypeName>::new_dvec3(self * other.f64())
            }
        }

        impl<A, B> std::ops::Mul<$crate::glam::DVec3> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            type Output = $crate::V3<$TypeName>;

            fn mul(self, other: $crate::glam::DVec3) -> Self::Output {
                $crate::V3::<$TypeName>::new_dvec3(other * self.f64())
            }
        }

        impl<A, B> std::ops::Div<$crate::Scalar> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            type Output = $TypeName;

            fn div(self, s: $crate::Scalar) -> Self {
                use $crate::Quantity;
                Self {
                    v: self.v / s.f64(),
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A, B> std::ops::DivAssign<$crate::Scalar> for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn div_assign(&mut self, s: $crate::Scalar) {
                use $crate::Quantity;
                self.v /= s.f64();
            }
        }

        impl<A, B> std::ops::Neg for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            type Output = $TypeName;

            fn neg(mut self) -> Self::Output {
                self.v = -self.v;
                self
            }
        }
    };

    ($TypeName:ty, $UnitA:path) => {
        impl<A> std::ops::Mul<$crate::Scalar> for $TypeName
        where
            A: $UnitA,
        {
            type Output = $TypeName;

            fn mul(self, s: $crate::Scalar) -> Self {
                use $crate::Quantity;
                Self {
                    v: self.v * s.f64(),
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A> std::ops::Mul<$TypeName> for $crate::Scalar
        where
            A: $UnitA,
        {
            type Output = $TypeName;

            fn mul(self, other: $TypeName) -> Self::Output {
                <$TypeName>::from(self.0.into_inner() * other.f64())
            }
        }

        impl<A> std::ops::MulAssign<$crate::Scalar> for $TypeName
        where
            A: $UnitA,
        {
            fn mul_assign(&mut self, s: $crate::Scalar) {
                use $crate::Quantity;
                self.v *= s.f64();
            }
        }

        impl<A> std::ops::Mul<$TypeName> for $crate::glam::DVec3
        where
            A: $UnitA,
        {
            type Output = $crate::V3<$TypeName>;

            fn mul(self, other: $TypeName) -> Self::Output {
                $crate::V3::<$TypeName>::new_dvec3(self * other.f64())
            }
        }

        impl<A> std::ops::Mul<$crate::glam::DVec3> for $TypeName
        where
            A: $UnitA,
        {
            type Output = $crate::V3<$TypeName>;

            fn mul(self, other: $crate::glam::DVec3) -> Self::Output {
                $crate::V3::<$TypeName>::new_dvec3(other * self.f64())
            }
        }

        impl<A> std::ops::Div<$crate::Scalar> for $TypeName
        where
            A: $UnitA,
        {
            type Output = $TypeName;

            fn div(self, s: $crate::Scalar) -> Self {
                use $crate::Quantity;
                Self {
                    v: self.v / s.f64(),
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A> std::ops::DivAssign<$crate::Scalar> for $TypeName
        where
            A: $UnitA,
        {
            fn div_assign(&mut self, s: $crate::Scalar) {
                use $crate::Quantity;
                self.v /= s.f64();
            }
        }

        impl<A> std::ops::Neg for $TypeName
        where
            A: $UnitA,
        {
            type Output = $TypeName;

            fn neg(mut self) -> Self::Output {
                self.v = -self.v;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! supports_cancellation {
    ($TypeNameSelf:ty, $TypeNameOther:ty, $UnitA:path, $UnitB:path) => {
        impl<A1, B1, A2, B2> std::ops::Div<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            B1: $UnitB,
            A2: $UnitA,
            B2: $UnitB,
        {
            type Output = $crate::Scalar;

            fn div(self, other: $TypeNameOther) -> Self::Output {
                Self::Output::from(self.v.0 / other.v.0)
            }
        }
    };

    ($TypeNameSelf:ty, $TypeNameOther:ty, $UnitA:path) => {
        impl<A1, A2> std::ops::Div<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            A2: $UnitA,
        {
            type Output = Scalar;

            fn div(self, other: $TypeNameOther) -> Self::Output {
                Self::Output::from(self.v.0 / other.v.0)
            }
        }
    };
}

#[macro_export]
macro_rules! supports_shift_ops {
    ($TypeNameSelf:ty, $TypeNameOther:ty, $UnitA:path, $UnitB:path) => {
        impl<A1, B1, A2, B2> std::ops::Add<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            B1: $UnitB,
            A2: $UnitA,
            B2: $UnitB,
        {
            type Output = $TypeNameSelf;

            fn add(self, other: $TypeNameOther) -> Self {
                Self {
                    v: self.v + <$TypeNameSelf>::from(&other).v,
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A1, B1, A2, B2> std::ops::AddAssign<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            B1: $UnitB,
            A2: $UnitA,
            B2: $UnitB,
        {
            fn add_assign(&mut self, other: $TypeNameOther) {
                self.v += <$TypeNameSelf>::from(&other).v;
            }
        }

        impl<A1, B1, A2, B2> std::ops::Sub<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            B1: $UnitB,
            A2: $UnitA,
            B2: $UnitB,
        {
            type Output = $TypeNameSelf;

            fn sub(self, other: $TypeNameOther) -> Self {
                Self {
                    v: self.v - <$TypeNameSelf>::from(&other).v,
                    phantom_1: PhantomData,
                    phantom_2: PhantomData,
                }
            }
        }

        impl<A1, B1, A2, B2> std::ops::SubAssign<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            B1: $UnitB,
            A2: $UnitA,
            B2: $UnitB,
        {
            fn sub_assign(&mut self, other: $TypeNameOther) {
                self.v -= <$TypeNameSelf>::from(&other).v;
            }
        }

        impl<A1, B1> $crate::num_traits::identities::Zero for $TypeNameSelf
        where
            A1: $UnitA,
            B1: $UnitB,
        {
            fn zero() -> Self {
                <$TypeNameSelf>::from(0f64)
            }

            fn is_zero(&self) -> bool {
                self.v == 0f64
            }
        }
    };

    ($TypeNameSelf:ty, $TypeNameOther:ty, $UnitA:path) => {
        impl<A1, A2> std::ops::Add<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            A2: $UnitA,
        {
            type Output = $TypeNameSelf;

            fn add(self, other: $TypeNameOther) -> Self {
                Self {
                    v: self.v + <$TypeNameSelf>::from(&other).v,
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A1, A2> std::ops::AddAssign<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            A2: $UnitA,
        {
            fn add_assign(&mut self, other: $TypeNameOther) {
                self.v += <$TypeNameSelf>::from(&other).v;
            }
        }

        impl<A1, A2> std::ops::Sub<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            A2: $UnitA,
        {
            type Output = $TypeNameSelf;

            fn sub(self, other: $TypeNameOther) -> Self {
                Self {
                    v: self.v - <$TypeNameSelf>::from(&other).v,
                    phantom_1: PhantomData,
                }
            }
        }

        impl<A1, A2> std::ops::SubAssign<$TypeNameOther> for $TypeNameSelf
        where
            A1: $UnitA,
            A2: $UnitA,
        {
            fn sub_assign(&mut self, other: $TypeNameOther) {
                self.v -= <$TypeNameSelf>::from(&other).v;
            }
        }

        impl<A1> $crate::num_traits::identities::Zero for $TypeNameSelf
        where
            A1: $UnitA,
        {
            fn zero() -> Self {
                <$TypeNameSelf>::from(0f64)
            }

            fn is_zero(&self) -> bool {
                self.v == 0f64
            }
        }
    };
}

#[macro_export]
macro_rules! supports_quantity_ops {
    ($TypeName:ty, $UnitA:path, $UnitB:path) => {
        impl<A, B> $crate::Quantity for $TypeName
        where
            A: $UnitA,
            B: $UnitB,
        {
            fn f64(&self) -> f64 {
                self.v.0
            }
        }
    };

    ($TypeName:ty, $UnitA:path) => {
        impl<A> $crate::Quantity for $TypeName
        where
            A: $UnitA,
        {
            fn f64(&self) -> f64 {
                self.v.0
            }
        }
    };
}
