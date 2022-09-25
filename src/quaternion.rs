use crate::vector::Vector;
use core::cmp::PartialEq;
use core::ops;

#[derive(Debug)]
pub struct Quaternion {
    a: f64, // constant
    b: f64, // i - x axis
    c: f64, // j - y axis
    d: f64, // k - z axis
}

impl Quaternion {
    // can be used to convert a vector in xyz space to a quaternion for easy rotation applications
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Quaternion {
            a: 0.,
            b: x,
            c: y,
            d: z,
        }
    }

    pub fn rotation(rotation_axis: &Vector, angle: f64) -> Self {
        // create identity quaternion of rotation axis with a = 0
        // return cos(angle) + sin(angle)(quaternion(axis)) representing rotation

        let q = Quaternion {
            a: 0.0,
            b: rotation_axis.x,
            c: rotation_axis.y,
            d: rotation_axis.z,
        };

        let rotation = q * angle.sin() + angle.cos();
        rotation
    }

    // convert quaternion back to 3d space vector (drops the constant a of quaternion ana maps b,c,d with x,y,z)
    pub fn to_vector(&self) -> Vector {
        Vector {
            x: self.b,
            y: self.c,
            z: self.d,
        }
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        let a = self.a * rhs.a - self.b * rhs.b - self.c * rhs.c - self.d * rhs.d;
        let b = self.a * rhs.b + self.b * rhs.a + self.c * rhs.d - self.d * rhs.c;
        let c = self.a * rhs.c - self.b * rhs.d + self.c * rhs.a + self.d * rhs.b;
        let d = self.a * rhs.d + self.b * rhs.c - self.c * rhs.b + self.d * rhs.a;

        Quaternion { a, b, c, d }
    }
}

impl ops::Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f64) -> Self::Output {
        Quaternion {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
            d: self.d * rhs,
        }
    }
}

impl ops::Add<f64> for Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: f64) -> Self::Output {
        Quaternion {
            a: self.a + rhs,
            b: self.b,
            c: self.c,
            d: self.d,
        }
    }
}

impl PartialEq<Quaternion> for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        (self.a - other.a).abs() <= f64::EPSILON
            && (self.b - other.b).abs() <= f64::EPSILON
            && (self.c - other.c).abs() <= f64::EPSILON
            && (self.d - other.d).abs() <= f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn quaternion_vector_multiplication() {
        let h1 = Quaternion {
            a: 1.0,
            b: 2.0,
            c: 3.0,
            d: 4.0,
        };
        let h2 = Quaternion {
            a: 1.0,
            b: 2.0,
            c: 3.0,
            d: 4.0,
        };

        let result = h1 * h2;
        assert_eq!(
            result,
            Quaternion {
                a: -28.0,
                b: 4.0,
                c: 6.0,
                d: 8.0
            }
        );
    }

    #[test]
    fn quaternion_scalar_multiplication() {
        let h1 = Quaternion {
            a: 0.0,
            b: 0.0,
            c: 1.0,
            d: 0.0,
        };

        assert_eq!(
            h1 * 2.0,
            Quaternion {
                a: 0.0,
                b: 0.0,
                c: 2.0,
                d: 0.0,
            }
        );
    }

    #[test]
    fn quaternion_scalar_addition() {
        let h1 = Quaternion {
            a: 0.0,
            b: 0.0,
            c: 1.0,
            d: 0.0,
        };

        assert_eq!(
            h1 + 2.0,
            Quaternion {
                a: 2.0,
                b: 0.0,
                c: 1.0,
                d: 0.0,
            }
        )
    }

    #[test]
    fn rotation() {
        let v = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        let rotation = Quaternion::rotation(&v, PI / 2.0);
        assert_eq!(
            rotation,
            Quaternion {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: 1.0
            }
        );

        let rotation = Quaternion::rotation(&v, -PI / 2.0);
        assert_eq!(
            rotation,
            Quaternion {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: -1.0
            }
        );
    }
}
