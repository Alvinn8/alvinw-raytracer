use crate::vector::Vec3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self { Self { origin, dir }}
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }

    pub fn origin(&self) -> Vec3 { self.origin }
    pub fn dir(&self) -> Vec3 { self.dir }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_test() {
        let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(ray.at(1.0), Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(ray.at(-1.0), Vec3::new(-1.0, -1.0, -1.0));
    }
}