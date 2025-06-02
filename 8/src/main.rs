use vm::{CodeWriter, Parser, VmTranslator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 2 {
        // read from file
        let path = std::path::Path::new(&args[1]);
        match path.extension().and_then(|s| s.to_str()) {
            Some("vm") => {}
            _ => {
                eprintln!("Error: The file must have a `.vm` extension");
                return Err("Invalid file extension".into());
            }
        }

        let file = std::fs::File::open(path)?;
        let parser = Parser::new(&mut std::io::BufReader::new(file));

        let ident = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("main")
            .to_string();
        let output_path = path.with_file_name(format!("{}.asm", ident));
        let output_file = std::fs::File::create(output_path)?;
        let writer = CodeWriter::new(std::io::BufWriter::new(output_file), ident);

        let mut translator = VmTranslator::new(parser, writer);
        translator.translate()?;
    } else {
        eprintln!("Error: missing file argument");
        return Err("Missing file argument".into());
    };

    Ok(())
}
