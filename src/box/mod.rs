use suoi_types::{Vector, Vector3};

use crate::{
    collision_shape::CollisionShape,
    plane::Plane,
    ray::{Ray, Raycast},
    rect::Rect,
};

pub struct Box {
    pub position: Vector3,
    pub size: Vector3,
}

impl CollisionShape for Box {
    fn raycast(&self, ray: &Ray) -> Raycast {
        let dir = ray.dir().unit();

        let plane = match dir.x > 0.0 {
            true => Plane::point_normal(
                self.position - Vector3::right() * self.size.x,
                -Vector3::right(),
            ),
            false => Plane::point_normal(
                self.position + Vector3::right() * self.size.x,
                Vector3::right(),
            ),
        };

        match plane.raycast(ray) {
            Raycast::Miss => (),
            Raycast::Hit(hit) => {
                let bounds = Rect {
                    position: self.position.yz(),
                    size: self.size.yz(),
                };
                if bounds.inside(hit.point.yz()) {
                    return Raycast::Hit(hit);
                }
            }
        };

        Raycast::Miss
    }
}
