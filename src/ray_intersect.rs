use raylib::prelude::Vector3;

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> bool;
}

