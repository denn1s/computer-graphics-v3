use raylib::prelude::Vector3;
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};

pub struct Cube {
    pub min: Vector3,
    pub max: Vector3,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vector3, size: f32, material: Material) -> Self {
        let half = size * 0.5;
        let half_v = Vector3::new(half, half, half);
        Cube {
            min: center - half_v,
            max: center + half_v,
            material,
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(
        &self,
        ray_origin: &Vector3,
        ray_direction: &Vector3,
        inv_dir: &Vector3,
    ) -> Intersect {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;
        let mut normal = Vector3::zero();

        // X slab
        if ray_direction.x.abs() < 1e-8 {
            if ray_origin.x < self.min.x || ray_origin.x > self.max.x {
                return Intersect::empty();
            }
        } else {
            let mut t1 = (self.min.x - ray_origin.x) * inv_dir.x;
            let mut t2 = (self.max.x - ray_origin.x) * inv_dir.x;
            let mut n = Vector3::new(-1.0, 0.0, 0.0);
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                n = Vector3::new(1.0, 0.0, 0.0);
            }
            if t1 > tmin {
                tmin = t1;
                normal = n;
            }
            tmax = tmax.min(t2);
            if tmin > tmax {
                return Intersect::empty();
            }
        }

        // Y slab
        if ray_direction.y.abs() < 1e-8 {
            if ray_origin.y < self.min.y || ray_origin.y > self.max.y {
                return Intersect::empty();
            }
        } else {
            let mut t1 = (self.min.y - ray_origin.y) * inv_dir.y;
            let mut t2 = (self.max.y - ray_origin.y) * inv_dir.y;
            let mut n = Vector3::new(0.0, -1.0, 0.0);
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                n = Vector3::new(0.0, 1.0, 0.0);
            }
            if t1 > tmin {
                tmin = t1;
                normal = n;
            }
            tmax = tmax.min(t2);
            if tmin > tmax {
                return Intersect::empty();
            }
        }

        // Z slab
        if ray_direction.z.abs() < 1e-8 {
            if ray_origin.z < self.min.z || ray_origin.z > self.max.z {
                return Intersect::empty();
            }
        } else {
            let mut t1 = (self.min.z - ray_origin.z) * inv_dir.z;
            let mut t2 = (self.max.z - ray_origin.z) * inv_dir.z;
            let mut n = Vector3::new(0.0, 0.0, -1.0);
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                n = Vector3::new(0.0, 0.0, 1.0);
            }
            if t1 > tmin {
                tmin = t1;
                normal = n;
            }
            tmax = tmax.min(t2);
            if tmin > tmax {
                return Intersect::empty();
            }
        }

        // Camera orbits outside the cubes, so only entry hits count.
        if tmin > 1e-4 {
            let point = *ray_origin + *ray_direction * tmin;
            // Material is Copy: no allocation, unlike the old String-based clone().
            return Intersect::new(point, normal, tmin, self.material);
        }

        Intersect::empty()
    }
}
