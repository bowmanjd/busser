use std::process;
mod cmd;

fn main() {
    if let Err(err) = cmd::run(argh::from_env()) {
        println!("{}", err);
        process::exit(1);
    }
}
