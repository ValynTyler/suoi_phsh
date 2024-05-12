use suoi_phsh::{collision_shape::CollisionShape, plane::Plane, ray::Ray};
use suoi_types::Vector3;

fn main() {
    let ray = Ray::point_dir(Vector3::up(), Vector3::up() * -1.);
    let plane = Plane::point_normal(Vector3::up() * -5.0, Vector3::up());

    println!("{:?}", plane.raycast(&ray));
}
