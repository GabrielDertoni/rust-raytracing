use std::fmt::{self, Display, Formatter};
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

use rand::{
    distributions::{Distribution, Standard},
    random, Rng,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub trait Num
where Self: Sized,
      Self: Add<Output = Self>,
      Self: Sub<Output = Self>,
      Self: Mul<Output = Self>,
      Self: Div<Output = Self>,
{}

impl<T> Num for T
where T: Sized,
      T: Add<Output = Self>,
      T: Sub<Output = Self>,
      T: Mul<Output = Self>,
      T: Div<Output = Self>,
{}


impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T: Num + Copy> Vec3<T> {
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn mag_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

macro_rules! impl_vec3_float {
    ($(impl Vec3<$ty:ident>;)+) => {
        $(impl_vec3_float!{ @once impl Vec3<$ty> })+
    };

    (@once impl Vec3<$ty:ident>) => {
        impl Vec3<$ty> {
            pub fn up() -> Self {
                Vec3::new(0.0, 1.0, 0.0)
            }

            pub fn mag(&self) -> $ty {
                self.mag_sq().sqrt()
            }

            pub fn unit(self) -> Vec3<$ty> {
                self / self.mag()
            }

            pub fn lerp(start: Self, end: Self, amnt: $ty) -> Self {
                start + (end - start) * amnt
            }

            pub fn random_in_range(min: Self, max: Self, rng: &mut impl Rng) -> Self {
                Self::lerp(min, max, rng.gen_range(0.0..1.0))
            }

            pub fn random_in_unit_disc() -> Self {
                let theta = random::<$ty>() * std::$ty::consts::TAU;
                let rho   = random::<$ty>();
                Vec3::new(
                    rho * theta.cos(),
                    rho * theta.sin(),
                    0.0
                )
            }

            pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Self {
                let phi   = rng.gen_range(0.0..std::$ty::consts::PI);
                let theta = rng.gen_range(0.0..std::$ty::consts::TAU);
                let rho   = rng.gen_range(0.0..1.0);
                Vec3::new(
                    rho * phi.sin() * theta.cos(),
                    rho * phi.sin() * theta.sin(),
                    rho * phi.cos(),
                )
            }

            pub fn random_unit(rng: &mut impl Rng) -> Self {
                Self::random_in_unit_sphere(rng).unit()
            }

            pub fn sqrt(&self) -> Self {
                Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
            }
        }
    }
}

impl_vec3_float! {
    impl Vec3<f32>;
    impl Vec3<f64>;
}

impl<T: Add> Add for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T::Output> {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub> Sub for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn sub(self, rhs: Vec3<T>) -> Vec3<T::Output> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Mul> Mul for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn mul(self, rhs: Vec3<T>) -> Vec3<T::Output> {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: Mul + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn mul(self, rhs: T) -> Vec3<T::Output> {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Div> Div for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn div(self, rhs: Vec3<T>) -> Vec3<T::Output> {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: Div + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn div(self, rhs: T) -> Vec3<T::Output> {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Neg> Neg for Vec3<T> {
    type Output = Vec3<T::Output>;

    fn neg(self) -> Vec3<T::Output> {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Distribution<Vec3<T>> for Standard
where
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }
}

// Implements arithmetic operations for wrapper types of Vec3
macro_rules! def_vec3_wrappers {
    ($($vis:vis struct $name:ident wrapper of Vec3<$ty:ty>;)+) => {
        $(def_vec3_wrappers!{ @once $vis struct $name wrapper of Vec3<$ty> })+
    };

    (@once $vis:vis struct $name:ident wrapper of Vec3<$ty:ty>) => {

        #[derive(Debug, Clone, Copy)]
        $vis struct $name(Vec3<$ty>);

        impl Deref for $name {
            type Target = Vec3<$ty>;

            fn deref(&self) -> &Vec3<$ty> {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Vec3<$ty> {
                &mut self.0
            }
        }

        impl From<Vec3<$ty>> for $name {
            fn from(val: Vec3<$ty>) -> $name {
                $name(val)
            }
        }

        impl Add for $name {
            type Output = $name;

            #[inline]
            fn add(self, rhs: $name) -> $name {
                $name(*self + *rhs)
            }
        }

        impl AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, rhs: $name) {
                *self.deref_mut() += *rhs
            }
        }

        impl Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(self, rhs: $name) -> $name {
                $name(*self - *rhs)
            }
        }

        impl SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                *self.deref_mut() -= *rhs
            }
        }

        impl Mul for $name {
            type Output = $name;

            #[inline]
            fn mul(self, rhs: $name) -> $name {
                $name(*self * *rhs)
            }
        }

        impl Mul<$ty> for $name {
            type Output = $name;

            #[inline]
            fn mul(self, rhs: $ty) -> $name {
                $name(*self * rhs)
            }
        }

        impl MulAssign<$ty> for $name {
            #[inline]
            fn mul_assign(&mut self, rhs: $ty) {
                *self.deref_mut() *= rhs
            }
        }

        impl Div for $name {
            type Output = $name;

            #[inline]
            fn div(self, rhs: $name) -> $name {
                $name(*self / *rhs)
            }
        }

        impl Div<$ty> for $name {
            type Output = $name;

            #[inline]
            fn div(self, rhs: $ty) -> $name {
                $name(*self / rhs)
            }
        }

        impl DivAssign<$ty> for $name {
            #[inline]
            fn div_assign(&mut self, rhs: $ty) {
                *self.deref_mut() /= rhs
            }
        }

        impl Neg for $name {
            type Output = $name;

            fn neg(self) -> $name {
                $name(-*self)
            }
        }

        impl Distribution<$name> for Standard
        where
            Standard: Distribution<$ty>,
        {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $name {
                $name(rng.gen())
            }
        }
    };
}

pub type Point3 = Vec3<f64>;

def_vec3_wrappers! {
    pub struct Color wrapper of Vec3<f64>;
}

impl Color {
    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn mid_gray() -> Color {
        Color::new(0.5, 0.5, 0.5)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    pub fn random() -> Color {
        Color::new(
            random::<f64>(),
            random::<f64>(),
            random::<f64>(),
        )
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> u8 {
        (self.x.clamp(0.0, 0.999) * 256.0) as u8
    }

    pub fn g(&self) -> u8 {
        (self.y.clamp(0.0, 0.999) * 256.0) as u8
    }

    pub fn b(&self) -> u8 {
        (self.z.clamp(0.0, 0.999) * 256.0) as u8
    }

    pub fn lerp(start: Color, end: Color, amnt: f64) -> Color {
        Color(Vec3::<f64>::lerp(*start, *end, amnt))
    }

    pub fn sqrt(&self) -> Color {
        Color::from(self.0.sqrt())
    }
}

impl Into<image::Rgb<u8>> for Color {
    fn into(self) -> image::Rgb<u8> {
        image::Rgb([self.r(), self.g(), self.b()])
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.r(),
            self.g(),
            self.b(),
        )
    }
}
