pub mod ast;
pub mod types;
pub use types::*;
pub mod codegen;

use std::fs;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::Path;
use std::process::Command;

pub fn run(input: &Root) {
    let output = &input.generator.output.value;

    let output_file_path = Path::new(output);

    if let Some(parent) = output_file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    let mut file = File::create(&output_file_path).expect("Failed to open file for codegen");

    file.write(b"// Code generated by Prisma Client Rust. DO NOT EDIT.\n\n")
        .unwrap();

    let client = codegen::generate_prisma_client(input);

    file.write(client.as_bytes()).unwrap();

    Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(output)
        .output()
        .unwrap();
}
