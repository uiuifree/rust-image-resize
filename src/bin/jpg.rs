use std::path::Path;
use uiuifree_image_convert::{ImageConvert, ImageFile, resize_and_webp};

fn main() {
    let output1200 = Path::new("./storage/example.jpg");
    let (file,webp) = resize_and_webp(output1200, 1200, "large").unwrap();
    let _ = ImageFile::new(output1200);
    println!("{:?}", file);
    println!("{:?}", webp);
    let _ = ImageConvert::new(Path::new(&file.url)).unwrap().write_png(Path::new("./storage/test.png"));
    let _ = resize_and_webp(Path::new(&file.url), 750, "medium");
    let _ = resize_and_webp(Path::new(&file.url), 500, "small");
    let _ = resize_and_webp(Path::new(&file.url), 245, "thumbnail");
}
