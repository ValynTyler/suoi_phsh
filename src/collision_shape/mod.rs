use crate::ray::{Ray, Raycast};

pub trait CollisionShape {
    fn raycast(&self, ray: &Ray) -> Raycast;
}
