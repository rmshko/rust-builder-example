mod checkpoint {
    pub struct Coordinates(pub f32, pub f32);

    pub struct Checkpoint {
        pub name: String,
        pub coords: Coordinates,
    }

    impl Checkpoint {
        pub fn new(name: String, coords: Coordinates) -> Self {
            Self { name, coords }
        }
    }
}

pub use checkpoint::{Checkpoint, Coordinates};