use godot::prelude::*;

pub mod boid;
pub use boid::Boid;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
