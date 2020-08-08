#[macro_use]
// mod macros;
// mod about_dialog;
// mod app;
// mod header_bar;
// mod overlay;
mod pipeline;
// mod settings;
// mod utils;

extern crate gstreamer as gst;
use crate::pipeline::Pipeline;

use gst::prelude::*;

fn tutorial_main() {
  // Initialize GStreamer
  gst::init().unwrap();

  // Create the pipeline and if that fail return
  let pipeline = Pipeline::new().map_err(|err| format!("Error creating pipeline: {:?}", err))?;


}

fn main() {
  tutorial_main();
}
