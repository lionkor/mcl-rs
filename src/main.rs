mod compiler;
mod interpreter;
mod op;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    compile: bool,
    #[arg(long, default_value_t = false)]
    exec: bool,
    #[arg(long, default_value_t = false)]
    optimize: bool,
    #[arg(long, default_value_t = false)]
    decompile: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut bytecode = vec![0x01 << 56];

    if args.compile {
        bytecode = compiler::compile(r#"
        push 2
        push 1
        print
        print
        "#)?;
    }

    if args.exec {
        let vm = interpreter::VM::new();
        vm.execute(bytecode)?;
    }

    Ok(())
}
