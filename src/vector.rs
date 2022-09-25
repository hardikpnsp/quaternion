use crate::quaternion::Quaternion;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn rotate(&mut self, rotation_axis: &Vector, angle: f64) {
        let identity_rotation_axis = &rotation_axis.convert_to_identity();

        let point = Quaternion::from_xyz(self.x, self.y, self.z);
        let rotation = Quaternion::rotation(identity_rotation_axis, angle / 2.0);
        let inverse_rotation = Quaternion::rotation(identity_rotation_axis, -angle / 2.0);

        let rotated_point = rotation * point * inverse_rotation;
        *self = rotated_point.to_vector();
    }

    pub fn convert_to_identity(self) -> Vector {
        self * (1.0 / self.modulo())
    }

    fn modulo(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }
}

impl PartialEq<Vector> for Vector {
    fn eq(&self, other: &Vector) -> bool {
        (self.x - other.x).abs() <= f64::EPSILON
            && (self.y - other.y).abs() <= f64::EPSILON
            && (self.z - other.z).abs() <= f64::EPSILON
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn vector_rotation() {
        // take a vector of length 1 on x axis
        let mut v = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        // apply a 90 degree rotation around z axis
        v.rotate(
            &Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            PI / 2.0,
        );

        // vector should now be on y axis
        assert_eq!(
            v,
            Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0
            }
        )
    }

    #[test]
    fn convert_to_identity() {
        let v = Vector {
            x: 4.0,
            y: 4.0,
            z: 2.0,
        };

        assert_eq!(
            v.convert_to_identity(),
            Vector {
                x: 4.0 / 6.0,
                y: 4.0 / 6.0,
                z: 2.0 / 6.0
            }
        )
    }
}
