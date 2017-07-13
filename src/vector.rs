#[derive(Clone, Copy)]
pub struct Vector3 {
    x : f64,
    y : f64,
    z : f64
}

impl Vector3 {
    pub fn dot (vec1 : Vector3, vec2 : Vector3) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
    }

    pub fn cross (u : Vector3, v : Vector3) -> Vector3 {
        Vector3 {x : u.y * v.z - u.z * v.y, y : u.z * v.x - u.x * v.z, z : u.x * v.y - u.y * v.x}
    }

    pub fn square_magnitude (&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    pub fn magnitude (&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    pub fn normalised (&self) -> Vector3 {
        let mag = self.magnitude();
        Vector3 {x : self.x / mag, y : self.y / mag, z : self.z / mag}
    }

    pub fn x_hat() -> Vector3 {
        Vector3 {x : 1.0, y : 0.0, z : 0.0}
    }

    pub fn y_hat() -> Vector3 {
        Vector3 {x : 0.0, y : 1.0, z : 0.0}
    }

    pub fn z_hat() -> Vector3 {
        Vector3 {x : 0.0, y : 0.0, z : 1.0}
    }

    pub fn zero() -> Vector3 {
        Vector3 {x : 0.0, y : 0.0, z : 0.0}
    }
}

#[cfg(test)]
mod test {
    use super::Vector3;
    
    #[test]
    fn dot_test() {
        let px = Vector3 {x : 1.0, y : 2.0, z: 3.0};
        let py = Vector3 {x : 4.0, y : 5.0, z: 6.0};
        assert!(Vector3::dot (px, py) == 32.0);
    }
}