mod checkpoint;
mod track_builder;
mod text_track_builder;
mod pen_sum_track_builder;
mod director;

use crate::text_track_builder::TextTrackBuilder;
use crate::pen_sum_track_builder::PenSumTrackBuilder;
use crate::director::Director;

fn main() {
    let text_result = Director::test_build(TextTrackBuilder::default());
    let pen_sum = Director::test_build(PenSumTrackBuilder::default());

    println!("Text track builder result:");
    println!("{}", &text_result);
    println!("");
    println!("Penalty sum track builder result:");
    println!("{}", &pen_sum);
}