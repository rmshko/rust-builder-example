mod director {
    use crate::checkpoint::{ Checkpoint, Coordinates };
    use crate::track_builder::TrackBuilder;

    pub struct Director;

    impl Director {
        pub fn test_build<B, T>(builder: B) -> T
        where B: TrackBuilder<T>
        {
            let cp1 = Checkpoint::new(String::from("name1"), Coordinates(15 as f32, 18 as f32));
            let cp2 = Checkpoint::new(String::from("name2"), Coordinates(35 as f32, 18 as f32));
            let cp3 = Checkpoint::new(String::from("name3"), Coordinates(32 as f32, 14 as f32));

            return builder
                .add_necessary_checkpoint(&cp1)
                .add_unnecessary_checkpoint(&cp2, 5)
                .add_unnecessary_checkpoint(&cp3, 13)
                .build();
        }
    }
}

pub use director::Director;