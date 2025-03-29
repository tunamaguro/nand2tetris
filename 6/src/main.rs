use std::io::Read;

use assembler::Assembler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let mut asm = if args.len() == 1 {
        let mut source = String::new();
        std::io::stdin().read_to_string(&mut source)?;
        Assembler::new(&source)
    } else if args.len() == 2 {
        let input = &args[1];
        let mut file = std::fs::File::open(input)?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        Assembler::new(&source)
    } else {
        panic!("need input path")
    };

    if args.len() == 2 {
        let mut out = std::io::stdout();
        asm.write(&mut out)?
    } else {
        let out_path = &args[3];
        let mut file = std::fs::File::open(out_path)?;

        asm.write(&mut file)?
    }

    Ok(())
}
