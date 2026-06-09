use crate::Matrix;

pub trait Tuple {
    fn x(&self) -> &f32;
    fn y(&self) -> &f32;
    fn z(&self) -> &f32;
    fn w(&self) -> &f32;
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self;
    fn transform(&self, transformations: &Matrix<4, 4>) -> Self
    where
        Self: Sized,
    {
        transformations * self
    }
}

#[macro_export]
macro_rules! impl_display_for_tuple {
    ($type:ty) => {
        impl Display for $type
        where
            $type: Tuple,
        {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "(x: {}, y: {}, z: {}, w: {})",
                    self.x(),
                    self.y(),
                    self.z(),
                    self.w()
                )
            }
        }
    };
}

#[macro_export]
macro_rules! impl_partial_eq_for_tuple {
    ($type: ty) => {
        impl PartialEq for $type
        where
            $type: Tuple,
        {
            fn eq(&self, other: &Self) -> bool {
                float_equals(&self.x, &other.x)
                    && float_equals(&self.y, &other.y)
                    && float_equals(&self.z, &other.z)
                    && float_equals(&self.w, &other.w)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_sub_for_tuple {
    ($type: ty) => {
        impl Sub for &$type
        where
            $type: Tuple,
        {
            type Output = Vector;

            fn sub(self, rhs: Self) -> Self::Output {
                Vector::new(
                    self.x - rhs.x,
                    self.y - rhs.y,
                    self.z - rhs.z,
                    self.w - rhs.w,
                )
            }
        }

        impl Sub for $type
        where
            $type: Tuple,
        {
            type Output = Vector;

            fn sub(self, rhs: Self) -> Self::Output {
                &self - &rhs
            }
        }
    };
}

#[macro_export]
macro_rules! impl_neg_for_tuple {
    ($type: ty) => {
        impl Neg for &$type
        where
            $type: Tuple,
        {
            type Output = $type;

            fn neg(self) -> Self::Output {
                <$type>::new(-self.x, -self.y, -self.z, -self.w)
            }
        }

        impl Neg for $type
        where
            $type: Tuple,
        {
            type Output = $type;

            fn neg(self) -> Self::Output {
                -&self
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mul_for_tuple {
    ($type: ty) => {
        impl Mul for &$type
        where
            $type: Tuple,
        {
            type Output = $type;

            fn mul(self, other: &$type) -> Self::Output {
                <$type>::new(
                    self.x * other.x,
                    self.y * other.y,
                    self.z * other.z,
                    self.w * other.w,
                )
            }
        }

        impl Mul for $type
        where
            $type: Tuple,
        {
            type Output = $type;

            fn mul(self, other: $type) -> Self::Output {
                &self * &other
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mul_f32_for_tuple {
    ($type: ty) => {
        impl Mul<f32> for &$type
        where
            $type: Tuple,
        {
            type Output = $type;

            fn mul(self, scalar: f32) -> Self::Output {
                Tuple::new(
                    self.x * scalar,
                    self.y * scalar,
                    self.z * scalar,
                    self.w * scalar,
                )
            }
        }
        impl Mul<f32> for $type
        where
            $type: Tuple,
        {
            type Output = $type;

            fn mul(self, scalar: f32) -> Self::Output {
                &self * scalar
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mul_matrix_for_tuple {
    ($type: ty) => {
        impl Mul<&Matrix<4, 4>> for &$type
        where
            $type: Tuple,
        {
            type Output = $type;

            fn mul(self, rhs: &Matrix<4, 4>) -> Self::Output {
                rhs * self
            }
        }
    };
}

#[macro_export]
macro_rules! impl_div_f32_for_tuple {
    ($type: ty) => {
        impl Div<f32> for &$type
        where
            $type: Tuple,
        {
            type Output = $type;

            fn div(self, fraction: f32) -> Self::Output {
                <$type>::new(
                    self.x / fraction,
                    self.y / fraction,
                    self.z / fraction,
                    self.w / fraction,
                )
            }
        }
    };
}
