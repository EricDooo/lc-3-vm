use lc3_vm::*;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <image-file>", args[0]);
        return Ok(());
    }

    let mut vm = LC3::new();
    vm.read_image_file(&args[1])?;
    vm.run();

    Ok(())
}
