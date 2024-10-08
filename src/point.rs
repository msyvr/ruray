use rand::Rng;

pub struct Point {
    x: f64;
    y: f64;
    z: f64;
}

impl Point {

    // New, at specific coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { 
            x, 
            y, 
            z,
        }
    }

    // New, at random coordinates within the world/scene.
    pub fn new_random(world_min: f64, world_max: f64) -> Point {
        Point::new(
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
    // Point/vector functions: {add, subtract, dot, cross}

    pub fn add(&self, other: &Point) -> Point (
        Point {
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        }
    )

    pub fn scale(&self, const: f64) -> Point (
        Point {
            self.x * const,
            self.y * const,
            self.z * const,
        }
    )

    pub fn subtract(&self, other: &Point) -> Point (
        self.add(&(other.scale(-1)))
    )    

    pub fn dot(&self, other: &Point) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(&self, other: &Point) -> Point {
        Point::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // Distance: to origin and also between two points.
    pub fn distance_origin(&self) -> f64 {
        dot(&self).sqrt()
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powf(2) + (self.y - other.y).powf(2) + (self.z - other.z).powf(2)).sqrt()
    }

    // TODO
    // A 'hit point' that's a surface will have a surface normal.
    pub fn surface_normal(&self, &Object) -> (Point, Point) {
        // return the start (at surface) and end of the surface
        // normal, taking care about direction

        // TODO implement
        unimplemented!("compute surface normal and return start/end points");
    }

}
