use crate::quaternion::Quaternion;

#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn rotate(&mut self, rotation_axis: &Vector, angle: f64) {
        let p = Quaternion::from_xyz(self.x, self.y, self.z);
        let q = Quaternion::rotation(rotation_axis, angle / 2.0);
        let qi = Quaternion::rotation(rotation_axis, -angle / 2.0);

        let rotated = q * p * qi;
        *self = rotated.to_vector();
    }
}

impl PartialEq<Vector> for Vector {
    fn eq(&self, other: &Vector) -> bool {
        (self.x - other.x).abs() <= f64::EPSILON
            && (self.y - other.y).abs() <= f64::EPSILON
            && (self.z - other.z).abs() <= f64::EPSILON
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
}
