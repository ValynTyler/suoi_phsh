use suoi_types::{Matrix4, Vector, Vector3};

use crate::{
    collision_shape::CollisionShape,
    plane::Plane,
    ray::{Ray, Raycast},
    rect::Rect,
};

pub struct BoundingBox {
    pub position: Vector3,
    pub size: Vector3,
}

impl BoundingBox {
    pub fn mat(&self) -> Matrix4 {
        &Matrix4::translate(self.position) * &Matrix4::scale(self.size)
    }
}

impl CollisionShape for BoundingBox {
    fn raycast(&self, ray: &Ray) -> Raycast {
        let dir = ray.dir().unit();

        // x axis
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

        // y axis
        let plane = match dir.y > 0.0 {
            true => Plane::point_normal(
                self.position - Vector3::up() * self.size.y,
                -Vector3::up(),
            ),
            false => Plane::point_normal(
                self.position + Vector3::up() * self.size.y,
                Vector3::up(),
            ),
        };

        match plane.raycast(ray) {
            Raycast::Miss => (),
            Raycast::Hit(hit) => {
                let bounds = Rect {
                    position: self.position.xz(),
                    size: self.size.xz(),
                };
                if bounds.inside(hit.point.xz()) {
                    return Raycast::Hit(hit);
                }
            }
        };

        // z axis
        let plane = match dir.z > 0.0 {
            true => Plane::point_normal(
                self.position - Vector3::fwd() * self.size.z,
                -Vector3::fwd(),
            ),
            false => Plane::point_normal(
                self.position + Vector3::fwd() * self.size.z,
                Vector3::fwd(),
            ),
        };

        match plane.raycast(ray) {
            Raycast::Miss => (),
            Raycast::Hit(hit) => {
                let bounds = Rect {
                    position: self.position.xy(),
                    size: self.size.xy(),
                };
                if bounds.inside(hit.point.xy()) {
                    return Raycast::Hit(hit);
                }
            }
        };
        
        Raycast::Miss
    }
}
