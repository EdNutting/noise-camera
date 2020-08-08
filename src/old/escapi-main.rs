extern crate escapi;
extern crate image;

/* "simplest", example of simply enumerating the available devices with ESCAPI */

fn main() {
    println!("devices: {}", escapi::num_devices());

    /* Set up capture parameters.
    */
    const DEV_INDEX: usize = 1;
    const W: u32 = 1920;
    const H: u32 = 1080;
    const FPS: u64 = 30;

    let camera = escapi::init(DEV_INDEX, W, H, FPS).expect("Could not initialize the camera");
    println!("capture initialized, device name: {}", camera.name());

    for i in 0..2 {
        println!("Frame #{}, captured and saved as image.png", i);
        let (width, height) = (camera.capture_width(), camera.capture_height());
        let pixels = camera.capture().expect("Could not capture an image");

        // Lets' convert it to RGB.
        let mut buffer = vec![0; width as usize * height as usize * 3];
        for j in 0..pixels.len() / 4 {
            buffer[j * 3] = pixels[j * 4 + 2];
            buffer[j * 3 + 1] = pixels[j * 4 + 1];
            buffer[j * 3 + 2] = pixels[j * 4];
        }

        image::save_buffer(format!("./images/image{i}.png", i=i),
                           &buffer,
                           width,
                           height,
                           image::ColorType::Rgb8).expect("Could not save an image");
    }

    println!("shutting down");
}
