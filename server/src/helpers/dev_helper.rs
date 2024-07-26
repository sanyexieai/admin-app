use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use dotenv::dotenv;

pub async fn get_dev(key: String, default_value: Option<String>) -> String {
    dotenv().ok();

    let value = match env::var(&key) {
        Ok(val) => val,
        Err(_) => {
            let file_path = ".env";
            
            // 判断 .env 文件是否存在
            if !std::path::Path::new(file_path).exists() {
                // 创建 .env 文件
                fs::File::create(file_path).expect("Failed to create .env file");
            }

            // 打开 .env 文件以追加内容
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_path)
                .expect("Failed to open .env file");

            // 使用提供的默认值或预定义的默认值
            let default_value = default_value.unwrap_or_else(|| "default_value".to_string());
            // 写入 key = value 到 .env 文件
            writeln!(file, "{} = {}", key, default_value).expect("Failed to write to .env file");
            // 返回默认值
            default_value
        }
    };

    value
}
