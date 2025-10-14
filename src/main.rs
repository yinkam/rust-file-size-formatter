use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(size: u64) -> Self {
        Sizes {
            bytes: format_size(FileSize::Bytes(size)),
            kilobytes: format_size(FileSize::Kilobytes(size as u64 / 1024)),
            megabytes: format_size(FileSize::Megabytes(size as u64 / 1_048_576)),
            gigabytes: format_size(FileSize::Gigabytes(size as u64 / 1_073_741_824)),
        }
    }
}

fn unit_scale(unit: &str) -> u64 {
    let unit = unit.to_lowercase();
    match unit.as_str() {
        "b" => 1,
        "kb" => 1024,
        "mb" => 1_048_576,
        "gb" => 1_073_741_824,
        _ => 1_073_741_824,
    }
}

fn format_size(filesize: FileSize) -> String {
    match filesize {
        FileSize::Bytes(b) => format!("{} bytes", b),
        FileSize::Kilobytes(kb) => format!("{} kilobytes", kb),
        FileSize::Megabytes(mb) => format!("{} megabytes", mb),
        FileSize::Gigabytes(gb) => format!("{} gigabytes", gb),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_size: Vec<&str> = args[1].split_whitespace().collect();
    let size: u64 = file_size[0].parse().unwrap();
    let unit = file_size[1];

    let scale = unit_scale(unit);
    let total_size = size * scale;

    println!("{:?}", Sizes::new(total_size));
}
