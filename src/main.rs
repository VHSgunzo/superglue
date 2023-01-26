use std::process::exit;
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

struct Cli {
    file_0_path: std::path::PathBuf,
    file_1_path: std::path::PathBuf,
}

static FILE_0_STR : &str = "small file (ELF binary runtime or Shell script)";
static FILE_1_STR : &str = "large file (SquashFS image or Archive)";

fn main() {

    let file_0_path = std::env::args().nth(1)
        .unwrap_or_else(|| {
            eprintln!("Specify {}!", FILE_0_STR);
            exit(1);
    });

    let file_1_path = std::env::args().nth(2)
        .unwrap_or_else(|| {
            eprintln!("Specify {}!", FILE_1_STR);
            exit(1);
    });

    let args = Cli {
        file_0_path: std::path::PathBuf::from(file_0_path),
        file_1_path: std::path::PathBuf::from(file_1_path),
    };

    let mut file_0 = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(args.file_0_path)
        .unwrap_or_else(|error| {
            eprintln!("{}: {}", FILE_0_STR, error);
            exit(1);
    });
    let file_0_size = file_0.metadata().unwrap().len();
    let file_0_type = file_0.metadata().unwrap().file_type();
    if ! file_0_type.is_file() {
        eprintln!("{} is not a file!", FILE_0_STR);
        exit(1);
    };

    let mut file_1 = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(args.file_1_path)
        .unwrap_or_else(|error| {
            eprintln!("{}: {}", FILE_1_STR, error);
            exit(1);
    });

    let file_1_size = file_1.metadata().unwrap().len();
    let file_1_type = file_1.metadata().unwrap().file_type();
    if ! file_1_type.is_file() {
        eprintln!("{} is not a file!", FILE_1_STR);
        exit(1);
    };

    if ! file_0_size <= file_1_size {
        eprintln!("It is necessary that {} is less than or equal to {}!", FILE_0_STR, FILE_1_STR);
        exit(1);
    };

    let mut tmp_data_0 = vec![0u8; file_0_size.try_into().unwrap()];
    file_0.read_exact(&mut tmp_data_0)
        .unwrap_or_else(|error| {
            eprintln!("{}: {}", FILE_0_STR, error);
            exit(1);
    });

    let parts_number = file_1_size / file_0_size;
    let parts_size = file_0_size * parts_number;
    let remains_part_size = file_1_size - parts_size;
    let last_part_size = file_0_size + remains_part_size;

    file_1.seek(SeekFrom::Start(file_1_size - last_part_size)).unwrap();
    let mut last_part_data = vec![0u8; last_part_size.try_into().unwrap()];
    file_1.read_exact(&mut last_part_data)
        .unwrap_or_else(|error| {
            eprintln!("{}: {}", FILE_0_STR, error);
            exit(1);
    });

    file_1.set_len(parts_size)
        .unwrap_or_else(|error| {
            eprintln!("{}: {}", FILE_1_STR, error);
            exit(1);
    });

    let mut seek_size = 0;
    file_1.seek(SeekFrom::Start(seek_size)).unwrap();
    for _ in 0..parts_number {
        let mut tmp_data_1 = vec![0u8; file_0_size.try_into().unwrap()];
        file_1.read_exact(&mut tmp_data_1)
            .unwrap_or_else(|error| {
                eprintln!("{}: {}", FILE_1_STR, error);
                exit(1);
        });
        file_1.seek(SeekFrom::Start(seek_size)).unwrap();
        file_1.write_all(&tmp_data_0)
            .unwrap_or_else(|error| {
                eprintln!("{}: {}", FILE_1_STR, error);
                exit(1);
        });
        tmp_data_0 = tmp_data_1;
        seek_size += file_0_size;
    }

    file_1.seek(SeekFrom::Start(seek_size)).unwrap();
    file_1.write_all(&last_part_data)
        .unwrap_or_else(|error| {
            eprintln!("{}: {}", FILE_1_STR, error);
            exit(1);
    });
}
