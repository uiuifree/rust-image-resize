use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use image::{DynamicImage,};
use image::imageops::FilterType;
use webp::Encoder;

#[derive(Default, Debug)]
pub struct ImageFile {
    pub ext: String,
    pub url: String,
    pub hash: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub size: u32,
    pub mine: String,
}

use sha3::{Digest, Sha3_256};

pub fn str_to_hash(value: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(value.as_bytes());
    let result = hasher.finalize();
    return format!("{:x}", result);
}

pub fn process(input: &Path, size: u32, name: &str) -> Result<ImageFile, ()> {
    let path = input.parent().unwrap().to_str().unwrap();
    let stem = input.file_stem().unwrap().to_str().unwrap();
    let ext = input.extension().unwrap().to_str().unwrap().to_lowercase();
    let uuid = str_to_hash(&path.to_string());


    let hash = format!("{}_{}_{}", name, stem, uuid);
    let output_origin = Path::new(path).join(format!("{}.{}", hash, ext));
    let output_webp = Path::new(path).join(format!("{}.webp", hash));

    let dynamic_image = resize(size, input, output_origin.as_path()).unwrap();
    let metadata = fs::metadata(output_origin.clone()).unwrap();
    convert_webp(dynamic_image.clone(), output_webp.as_path());

    let file = ImageFile {
        ext: ext.to_lowercase(),
        url: output_origin.to_str().unwrap_or_default().to_string(),
        hash: hash.to_string(),
        name: format!("{}.{}", stem, ext),
        width: dynamic_image.width(),
        height: dynamic_image.height(),
        size: metadata.size() as u32,
        mine: "".to_string(),
    };
    return Ok(file);
}


fn resize(target_size: u32, input: &Path, output: &Path) -> Result<DynamicImage, ()> {
    let img = image::open(input).unwrap();
    let width = img.width() as u32;
    let height = img.height() as u32;

    if width > target_size || height > target_size {
        let (target_width, target_height) =
            if width > height {
                let ratio: f32 = target_size as f32 / width as f32;
                (target_size, (height as f32 * ratio) as u32)
            } else {
                let ratio: f32 = target_size as f32 / height as f32;
                ((width as f32 * ratio) as u32, target_size)
            };
        let img = img.resize(target_width, target_height, FilterType::Lanczos3);
        img.save(output).unwrap();
        return Ok(img);
    } else {
        img.save(output).unwrap();
    }
    return Ok(img);
}

fn convert_webp(img: DynamicImage, output: &Path) {
    let enc = Encoder::from_image(&img).unwrap();
    let buf = enc.encode(75.0).to_vec();
    let webp = File::create(output).unwrap();
    BufWriter::new(webp).write(&buf).unwrap();
}
