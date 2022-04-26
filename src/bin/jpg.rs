use std::path::Path;
use image_convert::{ImageConverter, resize_and_webp};

fn main() {
    let output1200 = Path::new("./storage/example.jpg");
    let file = resize_and_webp(output1200, 1200, "large").unwrap();

    let _ = ImageConverter::new(Path::new(&file.url)).unwrap().write_png(Path::new("./storage/test.png"));
    let _ = resize_and_webp(Path::new(&file.url), 750, "medium");
    let _ = resize_and_webp(Path::new(&file.url), 500, "small");
    let _ = resize_and_webp(Path::new(&file.url), 245, "thumbnail");
}
