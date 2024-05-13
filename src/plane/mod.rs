use suoi_types::{Vector, Vector3};

use crate::{
    collision_shape::CollisionShape,
    ray::{HitInfo, Ray, Raycast},
};

const SIGNIFICANT_DIST_THRESH: f32 = 0.0000001;

pub struct Plane {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Plane {
    pub fn point_normal(point: Vector3, normal: Vector3) -> Self {
        Self {
            a: normal.x,
            b: normal.y,
            c: normal.z,
            d: -point.x * normal.x - point.y * normal.y - point.z * normal.z,
        }
    }

    pub fn normal(&self) -> Vector3 {
        Vector3 {
            x: self.a,
            y: self.b,
            z: self.c,
        }
    }
}

impl CollisionShape for Plane {
    fn raycast(&self, ray: &Ray) -> Raycast {
        let dir = ray.dir().unit();
        let abc = self.normal().unit();
        let dir_dot_abc = dir.dot(abc);

        match dir_dot_abc.abs() > SIGNIFICANT_DIST_THRESH {
            true => {
                let t = (-(ray.pos().dot(abc) + self.d)) / dir_dot_abc;
                let p = ray.pos() + dir * t;

                if t < 0.0 {
                    return Raycast::Miss    
                }

                Raycast::Hit(HitInfo {
                    distance: t,
                    point: p,
                    inside: dir_dot_abc > 0.0,
                })
            }
            false => Raycast::Miss,
        }
    }
}
