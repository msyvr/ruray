use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    // Modeling ray incremental evolution with dt ('time' increment).
    // Using 'at' just gives the position, so it's a Vec3, not another Ray.
    pub fn at(&self, dt: f64) -> Vec3 {
        self.origin + (self.direction * dt)
    }
}

#[cfg(test)]

mod test {
    use super::*;


    #[test]
    fn creates_new_ray() {
        let x: f64 = 1.0;
        let y: f64 = 1.0;
        let z: f64 = 1.0;
        let origin = Vec3::new(x, y, z);
        let direction = Vec3::new(x, y, z);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin.x(), x);
        assert_eq!(r.origin.y(), y);
        assert_eq!(r.origin.z(), z);
        
        assert_eq!(r.direction.x(), x);
        assert_eq!(r.direction.y(), y);
        assert_eq!(r.direction.z(), z);
    }

    #[test]
    fn evolves_ray() {
        let x: f64 = 1.0;
        let y: f64 = 1.0;
        let z: f64 = 1.0;
        let origin = Vec3::new(x, y, z);
        let direction = Vec3::new(x, y, z);
        let r = Ray::new(origin, direction);
        let position = r.at(1.0);

        assert_eq!(position.x(), x+x);
        assert_eq!(position.y(), y+y);
        assert_eq!(position.z(), z+z);
        
    }
}