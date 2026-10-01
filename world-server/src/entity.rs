use nalgebra::{Quaternion, Vector3};

/// An entity is anything in the simulation whose state may vary over time.
struct Entity {
    name: String,
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
}