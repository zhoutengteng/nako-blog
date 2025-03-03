use actix_multipart::form::tempfile::TempFile;
use nako_blog::boot;
use std::io::Write;
use tempfile::NamedTempFile;
use std::fs;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {


    // // 创建一个临时文件
    // let file = NamedTempFile::new()?;
    // let temp_path = file.into_temp_path();
    //
    // // 目标路径
    // let target_path = "./1.jpg";
    //
    // // 如果直接持久化失败，手动复制文件
    // if let Err(e) = fs::copy(&temp_path, &target_path) {
    //     eprintln!("跨磁盘复制失败: {}", e);
    //     return Err(e);
    // }
    //
    // // 复制成功后删除临时文件
    // if let Err(e) = fs::remove_file(&temp_path) {
    //     eprintln!("删除临时文件失败: {}", e);
    //     return Err(e);
    // }

    boot::app::start().await
}
