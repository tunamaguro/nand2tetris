fn clean_str(s: &str) -> &str {
    s.trim()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeGenError {
    InvalidInstruction,
}

pub fn dest(s: &str) -> Result<&str, CodeGenError> {
    dbg!(s);

    let bin = match clean_str(s) {
        "" => "000",
        "M" => "001",
        "D" => "010",
        "DM" => "011",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "ADM" => "111",
        _ => return Err(CodeGenError::InvalidInstruction),
    };
    Ok(bin)
}

pub fn comp(s: &str) -> Result<&str, CodeGenError> {
    let bin = match clean_str(s) {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "M+1" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" => "0000010",
        "D+M" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" => "0000000",
        "D&M" => "1000000",
        "D|A" => "0010101",
        "D|M" => "1010101",
        _ => return Err(CodeGenError::InvalidInstruction),
    };

    Ok(bin)
}

pub fn jump(s: &str) -> Result<&str, CodeGenError> {
    let bin = match clean_str(s) {
        "" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => return Err(CodeGenError::InvalidInstruction),
    };

    Ok(bin)
}
