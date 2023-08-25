mod cpu;
mod instruction;
mod stack;

use anyhow::bail;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::env;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("program must be called with a ROM file path as argument");
    }
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {} - {}",
                Local::now().format("%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    let mut cpu = cpu::Chip8::from_file(&args[1])?;
    cpu.run();
    cpu.dump_ram();
    Ok(())
}
