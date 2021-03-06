use length::astronomical::*;
use length::imperial::*;
use length::metric::*;

use mass::imperial::*;
use mass::metric::*;

use mass::imperial::Ton as ITon;
use mass::metric::Ton as MTon;

use time::*;

use temperature::celsius::*;
use temperature::fahrenheit::*;
use temperature::kelvin::*;

use core;
use core::marker::PhantomData;
use core::fmt;

pub trait Unit {
    fn new(val: f64) -> Self;

    //TODO: make inner() unsafe?
    fn inner(&self) -> f64;
}

pub trait UnitName {
    fn get_unit(&self) -> &'static str;
    fn get_unit_static() -> &'static str;
    fn write_unit_static(f: &mut fmt::Formatter) -> fmt::Result;
}

#[derive(Copy, Clone)]
pub struct Mul<T, U>(pub T, pub PhantomData<U>);

#[derive(Copy, Clone)]
pub struct Div<T, U>(pub T, pub PhantomData<U>);

impl<T, U> UnitName for Mul<T, U>
    where T: Unit + UnitName,
          U: UnitName
{
    fn get_unit(&self) -> &'static str {
        "mul-composite unit"
    }
    fn get_unit_static() -> &'static str {
        "mul-composite unit"
    }
    fn write_unit_static(f: &mut fmt::Formatter) -> fmt::Result {
        T::write_unit_static(f)?;
        write!(f, "*")?;
        U::write_unit_static(f)
    }
}

impl<T, U> fmt::Debug for Mul<T, U>
    where T: Unit + UnitName,
          U: UnitName
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.inner())?;
        T::write_unit_static(f)?;
        write!(f, "*")?;
        U::write_unit_static(f)
    }
}

impl<T, U> UnitName for Div<T, U>
    where T: Unit + UnitName,
          U: UnitName
{
    fn get_unit(&self) -> &'static str {
        "div-composite unit"
    }
    fn get_unit_static() -> &'static str {
        "div-composite unit"
    }
    fn write_unit_static(f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        T::write_unit_static(f)?;
        write!(f, ")/(")?;
        U::write_unit_static(f)?;
        write!(f, ")")
    }
}

impl<T, U> fmt::Debug for Div<T, U>
    where T: Unit + UnitName,
          U: UnitName
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (", self.inner())?;
        T::write_unit_static(f)?;
        write!(f, ")/(")?;
        U::write_unit_static(f)?;
        write!(f, ")")
    }
}

impl<T, U> Unit for Mul<T, U>
    where T: Unit
{
    #[inline(always)]
    fn new(val: f64) -> Mul<T, U> {
        Mul(T::new(val), PhantomData)
    }
    #[inline(always)]
    fn inner(&self) -> f64 {
        self.0.inner()
    }
}

impl<T, U> Unit for Div<T, U>
    where T: Unit
{
    #[inline(always)]
    fn new(val: f64) -> Div<T, U> {
        Div(T::new(val), PhantomData)
    }

    #[inline(always)]
    fn inner(&self) -> f64 {
        self.0.inner()
    }
}

impl<T, U> Div<T, U>
    where T: Unit,
          U: Unit
{
    #[inline(always)]
    pub fn multiply(self, other: U) -> T {
        T::new(self.0.inner() * other.inner())
    }
}

impl<T, U> Div<T, Mul<U, U>>
    where T: Unit,
          U: Unit
{
    #[inline(always)]
    pub fn integrate(self, other: U) -> Div<T, U> {
        Div(T::new(self.0.inner() * other.inner()), PhantomData)
    }
}

#[cfg(feature = "std")]
impl<T> Mul<T, T>
    where T: Copy + Unit
{
    #[inline(always)]
    pub fn sqrt(&self) -> T {
        extern crate std;
        T::new(self.inner().sqrt())
    }
}

impl<T, U> Mul<T, U>
    where T: Unit,
          U: Unit
{
    #[inline(always)]
    pub fn divide_right(self, other: U) -> T {
        T::new(self.0.inner() / other.inner())
    }

    #[inline(always)]
    pub fn divide_left(self, other: T) -> U {
        U::new(self.0.inner() / other.inner())
    }
}


impl<T, U, W, X> core::ops::Mul<Mul<T, U>> for Mul<W, X>
    where W: Copy + Unit,
          T: Copy + Unit
{
    type Output = Mul<W, Mul<X, Mul<T, U>>>;

    fn mul(self, other: Mul<T, U>) -> Self::Output {
        Mul(W::new(self.0.inner() * other.0.inner()), PhantomData)
    }
}

impl<T, U, W, X> core::ops::Mul<Div<T, U>> for Mul<W, X>
    where W: Copy + Unit,
          T: Copy + Unit
{
    type Output = Mul<W, Mul<X, Div<T, U>>>;

    fn mul(self, other: Div<T, U>) -> Self::Output {
        Mul(W::new(self.0.inner() * other.0.inner()), PhantomData)
    }
}

impl<T, U, W, X> core::ops::Mul<Mul<T, U>> for Div<W, X>
    where W: Copy + Unit,
          T: Copy + Unit
{
    type Output = Mul<W, Mul<T, Div<U, X>>>;

    fn mul(self, other: Mul<T, U>) -> Self::Output {
        Mul(W::new(self.0.inner() * other.0.inner()), PhantomData)
    }
}

impl<T, U, W, X> core::ops::Mul<Div<T, U>> for Div<W, X>
    where W: Copy + Unit,
          T: Copy + Unit
{
    type Output = Mul<W, Div<T, Mul<X, U>>>;

    fn mul(self, other: Div<T, U>) -> Self::Output {
        Mul(W::new(self.0.inner() * other.0.inner()), PhantomData)
    }
}


impl<T, U, W, X> core::ops::Div<Mul<T, U>> for Mul<W, X>
    where W: Copy + Unit,
          T: Copy + Unit
{
    type Output = Mul<W, Div<X, Mul<T, U>>>;

    fn div(self, other: Mul<T, U>) -> Self::Output {
        Mul(W::new(self.0.inner() / other.0.inner()), PhantomData)
    }
}

impl<T, U, W, X> core::ops::Div<Div<T, U>> for Mul<W, X>
    where W: Copy + Unit,
          T: Copy + Unit
{
    type Output = Mul<W, Mul<X, Div<U, T>>>;

    fn div(self, other: Div<T, U>) -> Self::Output {
        Mul(W::new(self.0.inner() / other.0.inner()), PhantomData)
    }
}

impl<T, U, W, X> core::ops::Div<Mul<T, U>> for Div<W, X>
    where W: Copy + Unit,
          T: Copy + Unit
{
    type Output = Div<W, Mul<X, Mul<T, U>>>;

    fn div(self, other: Mul<T, U>) -> Self::Output {
        Div(W::new(self.0.inner() / other.0.inner()), PhantomData)
    }
}

impl<T, U, W, X> core::ops::Div<Div<T, U>> for Div<W, X>
    where W: Copy + Unit,
          T: Copy + Unit
{
    type Output = Mul<W, Div<U, Mul<T, X>>>;

    fn div(self, other: Div<T, U>) -> Self::Output {
        Mul(W::new(self.0.inner() / other.0.inner()), PhantomData)
    }
}


impl<T, U> core::ops::Sub for Mul<T, U>
    where T: core::ops::Sub<T, Output = T>
{
    type Output = Mul<T, U>;

    fn sub(self, other: Mul<T, U>) -> Self::Output {
        return Mul(self.0 - other.0, self.1);
    }
}

impl<T, U> core::ops::Sub for Div<T, U>
    where T: core::ops::Sub<T, Output = T>
{
    type Output = Div<T, U>;

    fn sub(self, other: Div<T, U>) -> Self::Output {
        return Div(self.0 - other.0, self.1);
    }
}


impl<T, U> core::ops::Add for Mul<T, U>
    where T: core::ops::Add<T, Output = T>
{
    type Output = Mul<T, U>;

    fn add(self, other: Mul<T, U>) -> Self::Output {
        return Mul(self.0 + other.0, self.1);
    }
}

impl<T, U> core::ops::Add for Div<T, U>
    where T: core::ops::Add<T, Output = T>
{
    type Output = Div<T, U>;

    fn add(self, other: Div<T, U>) -> Self::Output {
        return Div(self.0 + other.0, self.1);
    }
}


impl<T, U> core::ops::Mul<f64> for Mul<T, U>
    where T: core::ops::Mul<f64, Output = T>
{
    type Output = Mul<T, U>;

    fn mul(self, other: f64) -> Self::Output {
        Mul(self.0 * other, self.1)
    }
}
impl<T, U> core::ops::Mul<Mul<T, U>> for f64
    where T: core::ops::Mul<f64, Output = T>
{
    type Output = Mul<T, U>;

    fn mul(self, other: Mul<T, U>) -> Self::Output {
        Mul(other.0 * self, other.1)
    }
}
impl<T, U> core::ops::Mul<i64> for Mul<T, U>
    where T: core::ops::Mul<f64, Output = T>
{
    type Output = Mul<T, U>;

    fn mul(self, other: i64) -> Self::Output {
        Mul(self.0 * other as f64, self.1)
    }
}
impl<T, U> core::ops::Mul<Mul<T, U>> for i64
    where T: core::ops::Mul<f64, Output = T>
{
    type Output = Mul<T, U>;

    fn mul(self, other: Mul<T, U>) -> Self::Output {
        Mul(other.0 * self as f64, other.1)
    }
}

impl<T, U> core::ops::MulAssign<f64> for Mul<T, U>
    where T: core::ops::MulAssign<f64>
{
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
    }
}
impl<T, U> core::ops::MulAssign<i64> for Mul<T, U>
    where T: core::ops::MulAssign<f64>
{
    fn mul_assign(&mut self, other: i64) {
        self.0 *= other as f64;
    }
}

impl<T, U> core::ops::Div<f64> for Mul<T, U>
    where T: core::ops::Div<f64, Output = T>
{
    type Output = Mul<T, U>;

    fn div(self, other: f64) -> Self::Output {
        Mul(self.0 / other, self.1)
    }
}
impl<T, U> core::ops::Div<i64> for Mul<T, U>
    where T: core::ops::Div<f64, Output = T>
{
    type Output = Mul<T, U>;

    fn div(self, other: i64) -> Self::Output {
        Mul(self.0 / other as f64, self.1)
    }
}

impl<T, U> core::ops::DivAssign<f64> for Mul<T, U>
    where T: core::ops::DivAssign<f64>
{
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
    }
}
impl<T, U> core::ops::DivAssign<i64> for Mul<T, U>
    where T: core::ops::DivAssign<f64>
{
    fn div_assign(&mut self, other: i64) {
        self.0 /= other as f64;
    }
}



impl<T, U> core::ops::Mul<f64> for Div<T, U>
    where T: core::ops::Mul<f64, Output = T>
{
    type Output = Div<T, U>;

    fn mul(self, other: f64) -> Self::Output {
        Div(self.0 * other, self.1)
    }
}
impl<T, U> core::ops::Mul<Div<T, U>> for f64
    where T: core::ops::Mul<f64, Output = T>
{
    type Output = Div<T, U>;

    fn mul(self, other: Div<T, U>) -> Self::Output {
        Div(other.0 * self, other.1)
    }
}
impl<T, U> core::ops::Mul<i64> for Div<T, U>
    where T: core::ops::Mul<f64, Output = T>
{
    type Output = Div<T, U>;

    fn mul(self, other: i64) -> Self::Output {
        Div(self.0 * other as f64, self.1)
    }
}
impl<T, U> core::ops::Mul<Div<T, U>> for i64
    where T: core::ops::Mul<f64, Output = T>
{
    type Output = Div<T, U>;

    fn mul(self, other: Div<T, U>) -> Self::Output {
        Div(other.0 * self as f64, other.1)
    }
}

impl<T, U> core::ops::MulAssign<f64> for Div<T, U>
    where T: core::ops::MulAssign<f64>
{
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
    }
}
impl<T, U> core::ops::MulAssign<i64> for Div<T, U>
    where T: core::ops::MulAssign<f64>
{
    fn mul_assign(&mut self, other: i64) {
        self.0 *= other as f64;
    }
}

impl<T, U> core::ops::Div<f64> for Div<T, U>
    where T: core::ops::Div<f64, Output = T>
{
    type Output = Div<T, U>;

    fn div(self, other: f64) -> Self::Output {
        Div(self.0 / other, self.1)
    }
}
impl<T, U> core::ops::Div<i64> for Div<T, U>
    where T: core::ops::Div<f64, Output = T>
{
    type Output = Div<T, U>;

    fn div(self, other: i64) -> Self::Output {
        Div(self.0 / other as f64, self.1)
    }
}

impl<T, U> core::ops::DivAssign<f64> for Div<T, U>
    where T: core::ops::DivAssign<f64>
{
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
    }
}
impl<T, U> core::ops::DivAssign<i64> for Div<T, U>
    where T: core::ops::DivAssign<f64>
{
    fn div_assign(&mut self, other: i64) {
        self.0 /= other as f64;
    }
}

impl_composite_base!(AU);
impl_composite_base!(Lightyear);
impl_composite_base!(Kilometer);
impl_composite_base!(Meter);
impl_composite_base!(Centimeter);
impl_composite_base!(Millimeter);
impl_composite_base!(Micrometer);
impl_composite_base!(Picometer);
impl_composite_base!(Femtometer);
impl_composite_base!(Inch);
impl_composite_base!(Foot);
impl_composite_base!(Yard);
impl_composite_base!(Mile);
impl_composite_base!(Fahrenheit);
impl_composite_base!(Celsius);
impl_composite_base!(Kelvin);
impl_composite_base!(Ounce);
impl_composite_base!(Pound);
impl_composite_base!(ITon);
impl_composite_base!(MTon);
impl_composite_base!(Kilogram);
impl_composite_base!(Gram);
impl_composite_base!(Centigram);
impl_composite_base!(Milligram);
impl_composite_base!(Second);
impl_composite_base!(Minute);
impl_composite_base!(Hour);
impl_composite_base!(Day);
impl_composite_base!(Year);
impl_composite_base!(Decade);
impl_composite_base!(Century);
impl_composite_base!(Millennium);

impl_composite!(Kilometer, Fahrenheit);
impl_composite!(Kilometer, Celsius);
impl_composite!(Kilometer, Kelvin);
impl_composite!(Kilometer, Ounce);
impl_composite!(Kilometer, Pound);
impl_composite!(Kilometer, ITon);
impl_composite!(Kilometer, MTon);
impl_composite!(Kilometer, Kilogram);
impl_composite!(Kilometer, Gram);
impl_composite!(Kilometer, Centigram);
impl_composite!(Kilometer, Milligram);
impl_composite!(Kilometer, Second);
impl_composite!(Kilometer, Minute);
impl_composite!(Kilometer, Hour);
impl_composite!(Kilometer, Day);
impl_composite!(Kilometer, Year);
impl_composite!(Kilometer, Decade);
impl_composite!(Kilometer, Century);
impl_composite!(Kilometer, Millennium);

impl_composite!(Meter, Fahrenheit);
impl_composite!(Meter, Celsius);
impl_composite!(Meter, Kelvin);
impl_composite!(Meter, Ounce);
impl_composite!(Meter, Pound);
impl_composite!(Meter, ITon);
impl_composite!(Meter, MTon);
impl_composite!(Meter, Kilogram);
impl_composite!(Meter, Gram);
impl_composite!(Meter, Centigram);
impl_composite!(Meter, Milligram);
impl_composite!(Meter, Second);
impl_composite!(Meter, Minute);
impl_composite!(Meter, Hour);
impl_composite!(Meter, Day);
impl_composite!(Meter, Year);
impl_composite!(Meter, Decade);
impl_composite!(Meter, Century);
impl_composite!(Meter, Millennium);

impl_composite!(Centimeter, Fahrenheit);
impl_composite!(Centimeter, Celsius);
impl_composite!(Centimeter, Kelvin);
impl_composite!(Centimeter, Ounce);
impl_composite!(Centimeter, Pound);
impl_composite!(Centimeter, ITon);
impl_composite!(Centimeter, MTon);
impl_composite!(Centimeter, Kilogram);
impl_composite!(Centimeter, Gram);
impl_composite!(Centimeter, Centigram);
impl_composite!(Centimeter, Milligram);
impl_composite!(Centimeter, Second);
impl_composite!(Centimeter, Minute);
impl_composite!(Centimeter, Hour);
impl_composite!(Centimeter, Day);
impl_composite!(Centimeter, Year);
impl_composite!(Centimeter, Decade);
impl_composite!(Centimeter, Century);
impl_composite!(Centimeter, Millennium);

impl_composite!(Millimeter, Fahrenheit);
impl_composite!(Millimeter, Celsius);
impl_composite!(Millimeter, Kelvin);
impl_composite!(Millimeter, Ounce);
impl_composite!(Millimeter, Pound);
impl_composite!(Millimeter, ITon);
impl_composite!(Millimeter, MTon);
impl_composite!(Millimeter, Kilogram);
impl_composite!(Millimeter, Gram);
impl_composite!(Millimeter, Centigram);
impl_composite!(Millimeter, Milligram);
impl_composite!(Millimeter, Second);
impl_composite!(Millimeter, Minute);
impl_composite!(Millimeter, Hour);
impl_composite!(Millimeter, Day);
impl_composite!(Millimeter, Year);
impl_composite!(Millimeter, Decade);
impl_composite!(Millimeter, Century);
impl_composite!(Millimeter, Millennium);

impl_composite!(Micrometer, Fahrenheit);
impl_composite!(Micrometer, Celsius);
impl_composite!(Micrometer, Kelvin);
impl_composite!(Micrometer, Ounce);
impl_composite!(Micrometer, Pound);
impl_composite!(Micrometer, ITon);
impl_composite!(Micrometer, MTon);
impl_composite!(Micrometer, Kilogram);
impl_composite!(Micrometer, Gram);
impl_composite!(Micrometer, Centigram);
impl_composite!(Micrometer, Milligram);
impl_composite!(Micrometer, Second);
impl_composite!(Micrometer, Minute);
impl_composite!(Micrometer, Hour);
impl_composite!(Micrometer, Day);
impl_composite!(Micrometer, Year);
impl_composite!(Micrometer, Decade);
impl_composite!(Micrometer, Century);
impl_composite!(Micrometer, Millennium);

impl_composite!(Picometer, Fahrenheit);
impl_composite!(Picometer, Celsius);
impl_composite!(Picometer, Kelvin);
impl_composite!(Picometer, Ounce);
impl_composite!(Picometer, Pound);
impl_composite!(Picometer, ITon);
impl_composite!(Picometer, MTon);
impl_composite!(Picometer, Kilogram);
impl_composite!(Picometer, Gram);
impl_composite!(Picometer, Centigram);
impl_composite!(Picometer, Milligram);
impl_composite!(Picometer, Second);
impl_composite!(Picometer, Minute);
impl_composite!(Picometer, Hour);
impl_composite!(Picometer, Day);
impl_composite!(Picometer, Year);
impl_composite!(Picometer, Decade);
impl_composite!(Picometer, Century);
impl_composite!(Picometer, Millennium);

impl_composite!(Femtometer, Fahrenheit);
impl_composite!(Femtometer, Celsius);
impl_composite!(Femtometer, Kelvin);
impl_composite!(Femtometer, Ounce);
impl_composite!(Femtometer, Pound);
impl_composite!(Femtometer, ITon);
impl_composite!(Femtometer, MTon);
impl_composite!(Femtometer, Kilogram);
impl_composite!(Femtometer, Gram);
impl_composite!(Femtometer, Centigram);
impl_composite!(Femtometer, Milligram);
impl_composite!(Femtometer, Second);
impl_composite!(Femtometer, Minute);
impl_composite!(Femtometer, Hour);
impl_composite!(Femtometer, Day);
impl_composite!(Femtometer, Year);
impl_composite!(Femtometer, Decade);
impl_composite!(Femtometer, Century);
impl_composite!(Femtometer, Millennium);

impl_composite!(AU, Fahrenheit);
impl_composite!(AU, Celsius);
impl_composite!(AU, Kelvin);
impl_composite!(AU, Ounce);
impl_composite!(AU, Pound);
impl_composite!(AU, ITon);
impl_composite!(AU, MTon);
impl_composite!(AU, Kilogram);
impl_composite!(AU, Gram);
impl_composite!(AU, Centigram);
impl_composite!(AU, Milligram);
impl_composite!(AU, Second);
impl_composite!(AU, Minute);
impl_composite!(AU, Hour);
impl_composite!(AU, Day);
impl_composite!(AU, Year);
impl_composite!(AU, Decade);
impl_composite!(AU, Century);
impl_composite!(AU, Millennium);

impl_composite!(Lightyear, Fahrenheit);
impl_composite!(Lightyear, Celsius);
impl_composite!(Lightyear, Kelvin);
impl_composite!(Lightyear, Ounce);
impl_composite!(Lightyear, Pound);
impl_composite!(Lightyear, ITon);
impl_composite!(Lightyear, MTon);
impl_composite!(Lightyear, Kilogram);
impl_composite!(Lightyear, Gram);
impl_composite!(Lightyear, Centigram);
impl_composite!(Lightyear, Milligram);
impl_composite!(Lightyear, Second);
impl_composite!(Lightyear, Minute);
impl_composite!(Lightyear, Hour);
impl_composite!(Lightyear, Day);
impl_composite!(Lightyear, Year);
impl_composite!(Lightyear, Decade);
impl_composite!(Lightyear, Century);
impl_composite!(Lightyear, Millennium);

impl_composite!(Inch, Fahrenheit);
impl_composite!(Inch, Celsius);
impl_composite!(Inch, Kelvin);
impl_composite!(Inch, Ounce);
impl_composite!(Inch, Pound);
impl_composite!(Inch, ITon);
impl_composite!(Inch, MTon);
impl_composite!(Inch, Kilogram);
impl_composite!(Inch, Gram);
impl_composite!(Inch, Centigram);
impl_composite!(Inch, Milligram);
impl_composite!(Inch, Second);
impl_composite!(Inch, Minute);
impl_composite!(Inch, Hour);
impl_composite!(Inch, Day);
impl_composite!(Inch, Year);
impl_composite!(Inch, Decade);
impl_composite!(Inch, Century);
impl_composite!(Inch, Millennium);

impl_composite!(Foot, Fahrenheit);
impl_composite!(Foot, Celsius);
impl_composite!(Foot, Kelvin);
impl_composite!(Foot, Ounce);
impl_composite!(Foot, Pound);
impl_composite!(Foot, ITon);
impl_composite!(Foot, MTon);
impl_composite!(Foot, Kilogram);
impl_composite!(Foot, Gram);
impl_composite!(Foot, Centigram);
impl_composite!(Foot, Milligram);
impl_composite!(Foot, Second);
impl_composite!(Foot, Minute);
impl_composite!(Foot, Hour);
impl_composite!(Foot, Day);
impl_composite!(Foot, Year);
impl_composite!(Foot, Decade);
impl_composite!(Foot, Century);
impl_composite!(Foot, Millennium);

impl_composite!(Yard, Fahrenheit);
impl_composite!(Yard, Celsius);
impl_composite!(Yard, Kelvin);
impl_composite!(Yard, Ounce);
impl_composite!(Yard, Pound);
impl_composite!(Yard, ITon);
impl_composite!(Yard, MTon);
impl_composite!(Yard, Kilogram);
impl_composite!(Yard, Gram);
impl_composite!(Yard, Centigram);
impl_composite!(Yard, Milligram);
impl_composite!(Yard, Second);
impl_composite!(Yard, Minute);
impl_composite!(Yard, Hour);
impl_composite!(Yard, Day);
impl_composite!(Yard, Year);
impl_composite!(Yard, Decade);
impl_composite!(Yard, Century);
impl_composite!(Yard, Millennium);

impl_composite!(Mile, Fahrenheit);
impl_composite!(Mile, Celsius);
impl_composite!(Mile, Kelvin);
impl_composite!(Mile, Ounce);
impl_composite!(Mile, Pound);
impl_composite!(Mile, ITon);
impl_composite!(Mile, MTon);
impl_composite!(Mile, Kilogram);
impl_composite!(Mile, Gram);
impl_composite!(Mile, Centigram);
impl_composite!(Mile, Milligram);
impl_composite!(Mile, Second);
impl_composite!(Mile, Minute);
impl_composite!(Mile, Hour);
impl_composite!(Mile, Day);
impl_composite!(Mile, Year);
impl_composite!(Mile, Decade);
impl_composite!(Mile, Century);
impl_composite!(Mile, Millennium);

impl_composite!(Fahrenheit, Ounce);
impl_composite!(Fahrenheit, Pound);
impl_composite!(Fahrenheit, ITon);
impl_composite!(Fahrenheit, MTon);
impl_composite!(Fahrenheit, Kilogram);
impl_composite!(Fahrenheit, Gram);
impl_composite!(Fahrenheit, Centigram);
impl_composite!(Fahrenheit, Milligram);
impl_composite!(Fahrenheit, Second);
impl_composite!(Fahrenheit, Minute);
impl_composite!(Fahrenheit, Hour);
impl_composite!(Fahrenheit, Day);
impl_composite!(Fahrenheit, Year);
impl_composite!(Fahrenheit, Decade);
impl_composite!(Fahrenheit, Century);
impl_composite!(Fahrenheit, Millennium);

impl_composite!(Celsius, Ounce);
impl_composite!(Celsius, Pound);
impl_composite!(Celsius, ITon);
impl_composite!(Celsius, MTon);
impl_composite!(Celsius, Kilogram);
impl_composite!(Celsius, Gram);
impl_composite!(Celsius, Centigram);
impl_composite!(Celsius, Milligram);
impl_composite!(Celsius, Second);
impl_composite!(Celsius, Minute);
impl_composite!(Celsius, Hour);
impl_composite!(Celsius, Day);
impl_composite!(Celsius, Year);
impl_composite!(Celsius, Decade);
impl_composite!(Celsius, Century);
impl_composite!(Celsius, Millennium);

impl_composite!(Kelvin, Ounce);
impl_composite!(Kelvin, Pound);
impl_composite!(Kelvin, ITon);
impl_composite!(Kelvin, MTon);
impl_composite!(Kelvin, Kilogram);
impl_composite!(Kelvin, Gram);
impl_composite!(Kelvin, Centigram);
impl_composite!(Kelvin, Milligram);
impl_composite!(Kelvin, Second);
impl_composite!(Kelvin, Minute);
impl_composite!(Kelvin, Hour);
impl_composite!(Kelvin, Day);
impl_composite!(Kelvin, Year);
impl_composite!(Kelvin, Decade);
impl_composite!(Kelvin, Century);
impl_composite!(Kelvin, Millennium);

impl_composite!(Ounce, Second);
impl_composite!(Ounce, Minute);
impl_composite!(Ounce, Hour);
impl_composite!(Ounce, Day);
impl_composite!(Ounce, Year);
impl_composite!(Ounce, Decade);
impl_composite!(Ounce, Century);
impl_composite!(Ounce, Millennium);

impl_composite!(Pound, Second);
impl_composite!(Pound, Minute);
impl_composite!(Pound, Hour);
impl_composite!(Pound, Day);
impl_composite!(Pound, Year);
impl_composite!(Pound, Decade);
impl_composite!(Pound, Century);
impl_composite!(Pound, Millennium);

impl_composite!(ITon, Second);
impl_composite!(ITon, Minute);
impl_composite!(ITon, Hour);
impl_composite!(ITon, Day);
impl_composite!(ITon, Year);
impl_composite!(ITon, Decade);
impl_composite!(ITon, Century);
impl_composite!(ITon, Millennium);

impl_composite!(MTon, Second);
impl_composite!(MTon, Minute);
impl_composite!(MTon, Hour);
impl_composite!(MTon, Day);
impl_composite!(MTon, Year);
impl_composite!(MTon, Decade);
impl_composite!(MTon, Century);
impl_composite!(MTon, Millennium);

impl_composite!(Kilogram, Second);
impl_composite!(Kilogram, Minute);
impl_composite!(Kilogram, Hour);
impl_composite!(Kilogram, Day);
impl_composite!(Kilogram, Year);
impl_composite!(Kilogram, Decade);
impl_composite!(Kilogram, Century);
impl_composite!(Kilogram, Millennium);

impl_composite!(Gram, Second);
impl_composite!(Gram, Minute);
impl_composite!(Gram, Hour);
impl_composite!(Gram, Day);
impl_composite!(Gram, Year);
impl_composite!(Gram, Decade);
impl_composite!(Gram, Century);
impl_composite!(Gram, Millennium);

impl_composite!(Centigram, Second);
impl_composite!(Centigram, Minute);
impl_composite!(Centigram, Hour);
impl_composite!(Centigram, Day);
impl_composite!(Centigram, Year);
impl_composite!(Centigram, Decade);
impl_composite!(Centigram, Century);
impl_composite!(Centigram, Millennium);

impl_composite!(Milligram, Second);
impl_composite!(Milligram, Minute);
impl_composite!(Milligram, Hour);
impl_composite!(Milligram, Day);
impl_composite!(Milligram, Year);
impl_composite!(Milligram, Decade);
impl_composite!(Milligram, Century);
impl_composite!(Milligram, Millennium);
