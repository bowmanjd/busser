use std::process;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod cmd;

fn main() {
    if let Err(err) = cmd::run(argh::from_env()) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
