use glib::variant::ToVariant;
use crate::pipeline::Pipeline;

use std::cell::RefCell;
use std::error;
use std::ops;
use std::rc::{Rc, Weak};

// Our refcounted application struct for containing all the state we have to carry around.
//
// This represents our main application window.
#[derive(Clone)]
pub struct App(Rc<AppInner>);

// Deref into the contained struct to make usage a bit more ergonomic
impl ops::Deref for App {
  type Target = AppInner;

  fn deref(&self) -> &AppInner {
    &*self.0
  }
}

// Weak reference to our application struct
//
// Weak references are important to prevent reference cycles. Reference cycles are cases where
// struct A references directly or indirectly struct B, and struct B references struct A again
// while both are using reference counting.
pub struct AppWeak(Weak<AppInner>);

impl AppWeak {
  // Upgrade to a strong reference if it still exists
  pub fn upgrade(&self) -> Option<App> {
    self.0.upgrade().map(App)
  }
}

pub struct AppInner {
  pipeline: Pipeline,

  timer: RefCell<Option<SnapshotTimer>>,
}

// Helper struct for the snapshot timer
//
// Allows counting down and removes the timeout source on Drop
struct SnapshotTimer {
  remaining: u32,
  // This needs to be Option because we need to be able to take
  // the value out in Drop::drop() removing the timeout id
  timeout_id: Option<glib::source::SourceId>,
}

impl SnapshotTimer {
  fn new(remaining: u32, timeout_id: glib::SourceId) -> Self {
    Self {
      remaining,
      timeout_id: Some(timeout_id),
    }
  }

  fn tick(&mut self) -> u32 {
    assert!(self.remaining > 0);
    self.remaining -= 1;

    self.remaining
  }
}

impl Drop for SnapshotTimer {
  fn drop(&mut self) {
    glib::source::source_remove(self.timeout_id.take().expect("No timeout id"));
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SnapshotState {
  Idle,
  TimerRunning,
}

impl<'a> From<&'a glib::Variant> for SnapshotState {
  fn from(v: &glib::Variant) -> SnapshotState {
    v.get::<bool>().expect("Invalid snapshot state type").into()
  }
}

impl From<bool> for SnapshotState {
  fn from(v: bool) -> SnapshotState {
    match v {
      false => SnapshotState::Idle,
      true => SnapshotState::TimerRunning,
    }
  }
}

impl From<SnapshotState> for glib::Variant {
  fn from(v: SnapshotState) -> glib::Variant {
    match v {
      SnapshotState::Idle => false.to_variant(),
      SnapshotState::TimerRunning => true.to_variant(),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RecordState {
  Idle,
  Recording,
}

impl<'a> From<&'a glib::Variant> for RecordState {
  fn from(v: &glib::Variant) -> RecordState {
    v.get::<bool>().expect("Invalid record state type").into()
  }
}

impl From<bool> for RecordState {
  fn from(v: bool) -> RecordState {
    match v {
      false => RecordState::Idle,
      true => RecordState::Recording,
    }
  }
}

impl From<RecordState> for glib::Variant {
  fn from(v: RecordState) -> glib::Variant {
    match v {
      RecordState::Idle => false.to_variant(),
      RecordState::Recording => true.to_variant(),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
  Quit,
  Settings,
  About,
  Snapshot(SnapshotState),
  Record(RecordState),
}

impl App {
  pub fn new() -> Result<App, Box<dyn error::Error>> {
    // Create the pipeline and if that fail return
    let pipeline = Pipeline::new().map_err(|err| format!("Error creating pipeline: {:?}", err))?;

    let app = App(Rc::new(AppInner {
      pipeline,
      timer: RefCell::new(None),
    }));

    // Create the application actions
    Action::create(&app);

    Ok(app)
  }

  // Downgrade to a weak reference
  pub fn downgrade(&self) -> AppWeak {
    AppWeak(Rc::downgrade(&self.0))
  }

  pub fn on_startup() {}

  // Called on the first application instance whenever the first application instance is started,
  // or any future second application instance
  pub fn on_activate(&self) {
    // Once the UI is shown, start the GStreamer pipeline. If
    // an error happens, we immediately shut down
    if let Err(err) = self.pipeline.start() {
      panic!(format!("Failed to set pipeline to playing: {}", err));
    }
  }

  // Called when the application shuts down. We drop our app struct here
  pub fn on_shutdown(self) {
    // This might fail but as we shut down right now anyway this doesn't matter
    // TODO: If a recording is currently running we would like to finish that first
    // before quitting the pipeline and shutting down the pipeline.
    let _ = self.pipeline.stop();
  }

  // When the record button is clicked it triggers the record action, which will call this.
  // We have to start or stop recording here
  pub fn on_record_state_changed(&self, new_state: RecordState) {
    // Start/stop recording based on button active'ness
    match new_state {
      RecordState::Recording => {
        if let Err(err) = self.pipeline.start_recording() {
          panic!(format!("Failed to start recording: {}", err));
        }
      }
      RecordState::Idle => self.pipeline.stop_recording(),
    }
  }
}

impl Action {
  // The full action name as is used in e.g. menu models
  pub fn full_name(self) -> &'static str {
    match self {
      Action::Quit => "app.quit",
      Action::Settings => "app.settings",
      Action::About => "app.about",
      Action::Snapshot(_) => "app.snapshot",
      Action::Record(_) => "app.record",
    }
  }

  // Create our application actions here
  //
  // These are connected to our buttons and can be triggered by the buttons, as well as remotely
  fn create(app: &App) {
    // TODO:
    // app.on_record_state_changed(state.into());
  }
}
