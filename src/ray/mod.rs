use suoi_types::Vector3;

use crate::collision_shape::CollisionShape;

#[derive(Debug, Clone, Copy)]
pub enum Raycast {
    Miss,
    Hit(HitInfo),
}

impl Raycast {
    pub fn is_hit(&self) -> bool {
        match self {
            Raycast::Miss => false,
            Raycast::Hit(_) => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HitInfo {
    pub distance: f32,
    pub point: Vector3,
    pub inside: bool,
}

pub struct Ray {
    point: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn point_dir(point: Vector3, direction: Vector3) -> Self {
        Self { point, direction }
    }

    pub fn dir(&self) -> Vector3 {
        self.direction
    }

    pub fn pos(&self) -> Vector3 {
        self.point
    }

    pub fn cast(&self, targets: Vec<&dyn CollisionShape>) -> Raycast {
        let mut closest: Option<HitInfo> = None;

        for t in targets {
            match t.raycast(self) {
                Raycast::Miss => (),
                Raycast::Hit(info) => match closest {
                    Some(hit) => {
                        if info.distance < hit.distance {
                            closest = Some(info)
                        }
                    }
                    None => {
                        closest = Some(info);
                    }
                },
            }
        }

        match closest {
            Some(info) => Raycast::Hit(info),
            None => Raycast::Miss,
        }
    }

    pub fn cast_all(&self, _targets: Vec<&dyn CollisionShape>) -> Vec<Raycast> {
        todo!()
    }
}
