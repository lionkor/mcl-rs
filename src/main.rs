mod compiler;
mod interpreter;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = true)]
    compile: bool,
    #[arg(long, default_value_t = true)]
    exec: bool,
    #[arg(long, default_value_t = true)]
    optimize: bool,
    #[arg(long, default_value_t = false)]
    decompile: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut bytecode = vec![0x01 << 56];

    if args.compile && false{
        bytecode = compiler::compile("push 1")?;
    }

    if args.exec {
        let vm = interpreter::VM::new();
        vm.execute(bytecode)?;
    }

    Ok(())
}
