use std::path::Path;
use image_convert::process;

fn main() {
    let output1200 = Path::new("./storage/example.jpg");
    let file = process(output1200, 1200, "large").unwrap();
    let _ = process(Path::new(&file.url), 750, "medium");
    let _ = process(Path::new(&file.url), 500, "small");
    let _ = process(Path::new(&file.url), 245, "thumbnail");
}
