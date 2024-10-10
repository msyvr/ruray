use rand::Rng;
use std::f64;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {

    // New, at specific coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { 
            x, 
            y, 
            z,
        }
    }

    // New, at random coordinates within the world/scene.
    
    pub fn new_random(world_min: f64, world_max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(
            rng.gen_range(world_min..world_max),
            rng.gen_range(world_min..world_max),
            rng.gen_range(world_min..world_max),
        )
    }

    // Components along axes.

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    // Ray approximation: ray === straight line between two points. 

    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // Distance: to origin and also between two points.
    pub fn distance_origin(&self) -> f64 {
        self.dot(&self).sqrt()
    }

    pub fn distance(&self, other: &Vec3) -> f64 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0)).sqrt()
    }

}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn creates_new_point() {
        let x = 3.0;
        let y = 4.0;
        let z = 5.0;
        let p = Vec3::new(x, y, z);

        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
        assert_eq!(p.z, z);
    }

    #[test]
    fn  creates_random_point() {
        let p = Vec3::new_random(-1.0, 1.0);
        assert!(p.x() >= -1.0 && p.x() <= 1.0);
        assert!(p.y() >= -1.0 && p.y() <= 1.0);
        assert!(p.z() >= -1.0 && p.z() <= 1.0);
    }

    #[test]
    fn computes_dot_product() {
        let p1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let p2: Vec3 = Vec3::new(3.0, 2.0, 1.0);
        let dot = p1.dot(&p2);

        assert_eq!(dot, 10.0);
    }

    #[test]
    fn computes_cross_product() {
        let p1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let p2: Vec3 = Vec3::new(3.0, 2.0, 1.0);
        let p3 = Vec3::new(-4.0, 8.0, -4.0);
        let cross = p1.cross(&p2);

        assert_eq!(p3.x(), cross.x());
        assert_eq!(p3.y(), cross.y());
        assert_eq!(p3.z(), cross.z());
    }  

    #[test]
    fn computes_distance_to_origin() {
        let p = Vec3{x:3.0, y:4.0, z:0.0};
        let distance = p.distance_origin();
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn computes_distance() {
        let p1 = Vec3{x:3.0, y:4.0, z:0.0};
        let p2 = Vec3{x:0.0, y:0.0, z:0.0};
        let distance12 = p1.distance(&p2);
        assert_eq!(distance12, 5.0);
        let p11 = Vec3{x:4.0, y:5.0, z:1.0};
        let p22 = Vec3{x:1.0, y:1.0, z:1.0};
        let distance1122 = p11.distance(&p22);
        assert_eq!(distance1122, 5.0);        
    }    
}
