mod pen_sum_track_builder {
    use crate::track_builder::TrackBuilder;
    use crate::checkpoint::Checkpoint;

    pub struct PenSumTrackBuilder {
        penalty_sum: u32
    }

    impl Default for PenSumTrackBuilder {
        fn default() -> Self {
            return Self {
                penalty_sum: 0
            }
        }
    }

    impl TrackBuilder<u32> for PenSumTrackBuilder {
        fn add_necessary_checkpoint(self, _cp: &Checkpoint) -> Self {
            return self;
        }

        fn add_unnecessary_checkpoint(mut self, _cp: &Checkpoint, penalty: u16) -> Self {
            self.penalty_sum += penalty as u32;

            return self;
        }

        fn build(self) -> u32 {
            return self.penalty_sum;
        }
    }
}

pub use pen_sum_track_builder::PenSumTrackBuilder;