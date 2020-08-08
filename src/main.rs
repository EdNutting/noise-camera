use std::path::Path;

extern crate camera_capture;

fn main() {
  let cam = camera_capture::create(0).unwrap();
  let cam = cam.fps(5.0).unwrap().start().unwrap();
  for image in cam {
    let path = Path::new("./image.jpg");
    let _ = image.save(path);
    println!("frame");
    break;
  }
  println!("done");
}
