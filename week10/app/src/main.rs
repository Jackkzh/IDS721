use std::env;
use std::path::Path;
use image::{DynamicImage, GenericImageView};

fn main() {
    // 从命令行参数中获取图片路径
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];

    // 检查图片格式，只支持 jpg 格式
    let extension = Path::new(image_path)
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase();
    if extension != "jpg" {
        eprintln!("只支持 jpg 格式的图片");
        std::process::exit(1);
    }

    // 加载图片
    let img = match image::open(image_path) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("无法打开图片: {}", err);
            std::process::exit(1);
        }
    };

    // 获取原始图片的尺寸
    let (width, height) = img.dimensions();

    // 压缩图片并保存
    let compressed = img.resize(width / 2, height / 2, image::imageops::FilterType::Lanczos3);
    let compressed_path = format!("{}_compressed.jpg", image_path.trim_end_matches(".jpg"));
    match compressed.save(&compressed_path) {
        Ok(_) => println!("图片已保存为: {}", compressed_path),
        Err(err) => eprintln!("无法保存图片: {}", err),
    }
}
