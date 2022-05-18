//! WEB系の開発でよく使う画像のリサイズや変換を行うライブラリです。
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use image::{ColorType, DynamicImage, ImageEncoder};
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use webp::Encoder;
use sha3::{Digest, Sha3_256};

/// 画像の構造体
#[derive(Default, Debug)]
pub struct ImageFile {
    /// 拡張子
    pub ext: String,
    /// 保存URL
    pub url: String,
    /// hash付ファイル名
    pub hash: String,
    /// ファイル名
    pub name: String,
    /// 画像横幅
    pub width: u32,
    /// 画像高さ
    pub height: u32,
    /// ファイルサイズ
    pub size: u64,
    /// ファイルフォーマット
    pub mine: String,
}

/// 画像変換処理
pub struct ImageConvert {
    image: DynamicImage,
}

impl ImageConvert {
    pub fn new(input: &Path) -> Result<ImageConvert, String> {
        let img = image::open(input);
        if img.is_err() {
            return Err(img.unwrap_err().to_string());
        }
        Ok(ImageConvert {
            image: img.unwrap()
        })
    }
    pub fn from_image(image: DynamicImage) -> ImageConvert {
        ImageConvert {
            image
        }
    }
    /// 最大横幅を指定し、設定値以下にリサイズ
    pub fn resize(self, target_size: u32, output: &Path) -> Result<DynamicImage, ()> {
        let img = self.image;
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
        return Ok(img.clone());
    }
    /// webp書き出し
    pub fn write_webp(self, output: &Path) -> Result<(), ()> {
        let img = self.image;
        let enc = Encoder::from_image(&img);
        if enc.is_err() {
            return Err(());
        }
        let buf = enc.unwrap().encode(75.0).to_vec();
        let webp = File::create(output).unwrap();
        BufWriter::new(webp).write(&buf).unwrap();
        Ok(())
    }
    /// png書き出し
    pub fn write_png(self, output: &Path) -> Result<(), ()> {
        let img = self.image;
        let rgb8 = img.to_rgb8().to_vec();
        let mut buf = Vec::new();
        let enc = PngEncoder::new(&mut buf).write_image(&rgb8, img.width(), img.height(), ColorType::Rgb8);
        if enc.is_err() {
            return Err(());
        }
        let webp = File::create(output).unwrap();
        BufWriter::new(webp).write(&buf).unwrap();
        Ok(())
    }
}


fn str_to_hash(value: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(value.as_bytes());
    let result = hasher.finalize();
    return format!("{:x}", result);
}

/// リサイズとwebp変換を一括で実行するメソッド
pub fn resize_and_webp(input: &Path, size: u32, name: &str) -> Result<ImageFile, ()> {
    let path = input.parent().unwrap().to_str().unwrap();
    let stem = input.file_stem().unwrap().to_str().unwrap();
    let ext = input.extension().unwrap().to_str().unwrap().to_lowercase();
    let uuid = str_to_hash(&path.to_string());


    let hash = format!("{}_{}_{}", name, stem, uuid);
    let output_origin = Path::new(path).join(format!("{}.{}", hash, ext));
    let output_webp = Path::new(path).join(format!("{}.webp", hash));

    let dynamic_image = ImageConvert::new(input).unwrap().resize(size, output_origin.as_path()).unwrap();
    let metadata = fs::metadata(output_origin.clone()).unwrap();
    let _ = ImageConvert::from_image(dynamic_image.clone()).write_webp(output_webp.as_path());
    let file = ImageFile {
        ext: ext.to_lowercase(),
        url: output_origin.to_str().unwrap_or_default().to_string(),
        hash: hash.to_string(),
        name: format!("{}.{}", stem, ext),
        width: dynamic_image.width(),
        height: dynamic_image.height(),
        size: metadata.size() as u64,
        mine: "".to_string(),
    };
    return Ok(file);
}

impl ImageFile {
    pub fn new(input: &Path) -> Result<ImageFile, ()> {
        let path = input.parent().unwrap().to_str().unwrap();
        let stem = input.file_stem().unwrap().to_str().unwrap();
        let ext = input.extension().unwrap().to_str().unwrap().to_lowercase();
        let uuid = str_to_hash(&path.to_string());
        let hash = format!("{}_{}", stem, uuid);
        let dynamic_image = ImageConvert::new(input).unwrap().image;
        let metadata = fs::metadata(input.clone()).unwrap();
        return Ok(ImageFile {
            ext: ext.to_lowercase(),
            url: input.to_str().unwrap_or_default().to_string(),
            hash: hash.to_string(),
            name: format!("{}.{}", stem, ext),
            width: dynamic_image.width(),
            height: dynamic_image.height(),
            size: metadata.size() as u64,
            mine: "".to_string(),
        });
    }
}