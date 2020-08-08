#[macro_use]
mod macros;
mod app;
mod pipeline;
mod settings;
mod utils;

use std::error;

use crate::app::App;

// Unique application name to identify it
//
// This is used for ensuring that there's only ever a single instance of our application
pub const APPLICATION_NAME: &str = "com.github.gtk-rs.cameraview";

fn main() -> Result<(), Box<dyn error::Error>> {
    // Initialize GStreamer. This checks, among other things, what plugins are available
    gst::init()?;

    Ok(())
}
