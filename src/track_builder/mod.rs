mod track_builder {
    use crate::checkpoint::Checkpoint;

    pub trait TrackBuilder<T>  {
        fn add_necessary_checkpoint(self, cp: &Checkpoint) -> Self;
        fn add_unnecessary_checkpoint(self, cp: &Checkpoint, penalty: u16) -> Self;
        fn build(self) -> T;
    }
}

pub use track_builder::TrackBuilder;