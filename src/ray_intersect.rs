use raylib::prelude::Vector3;
use crate::material::Material;

// Copy: no heap, cheap to return by value (fixes the old clone() cost).
#[derive(Clone, Copy)]
pub struct Intersect {
    pub point: Vector3,
    pub normal: Vector3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(point: Vector3, normal: Vector3, distance: f32, material: Material) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            point: Vector3::zero(),
            normal: Vector3::zero(),
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
        }
    }
}

pub trait RayIntersect {
    // inv_dir = 1.0 / ray_direction, precomputed once per ray by the caller
    // so we don't pay 3 divisions per object per ray.
    fn ray_intersect(
        &self,
        ray_origin: &Vector3,
        ray_direction: &Vector3,
        inv_dir: &Vector3,
    ) -> Intersect;
}
