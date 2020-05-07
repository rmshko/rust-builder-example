mod text_track_builder {
    use crate::track_builder::TrackBuilder;
    use crate::checkpoint::Checkpoint;

    pub struct TextTrackBuilder {
        pub string_representation: String
    }

    impl Default for TextTrackBuilder {
        fn default() -> Self {
            return Self {
                string_representation: String::from("Type    Name    Coords    Penalty\n_________________________________")
            };
        }
    }

    impl TrackBuilder<String> for TextTrackBuilder {
        fn add_necessary_checkpoint(mut self, cp: &Checkpoint) -> Self {
            self.string_representation = format!("{}\n{}    {}     {}     {}", self.string_representation, "NEC", &cp.name, format!("({}, {})", &cp.coords.0, &cp.coords.1), "Failure");

            return self;
        }

        fn add_unnecessary_checkpoint(mut self, cp: &Checkpoint, penalty: u16) -> Self {
            self.string_representation = format!("{}\n{}    {}     {}     {}", self.string_representation, "UNNEC", &cp.name, format!("({}, {})", &cp.coords.0, &cp.coords.1), penalty);

            return self;
        }

        fn build(self) -> String {
            return self.string_representation;
        }
    }
}

pub use text_track_builder::TextTrackBuilder;