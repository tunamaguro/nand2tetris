use std::io;

mod code;
mod parser;

use parser::{InstructionType, Parser};

struct SymbolTable {
    map: std::collections::BTreeMap<String, usize>,
}

impl SymbolTable {
    fn new() -> Self {
        const DEFAULT_SYMBOL: &[(&str, usize)] = &[
            ("R0", 0),
            ("R1", 1),
            ("R2", 2),
            ("R3", 3),
            ("R4", 4),
            ("R5", 5),
            ("R6", 6),
            ("R7", 7),
            ("R8", 8),
            ("R9", 9),
            ("R10", 10),
            ("R11", 11),
            ("R12", 12),
            ("R13", 13),
            ("R14", 14),
            ("R15", 15),
            ("SP", 0),
            ("LCL", 1),
            ("ARG", 2),
            ("THIS", 3),
            ("THAT", 4),
            ("SCREEN", 16384),
            ("KBD", 24576),
        ];

        let map = DEFAULT_SYMBOL
            .iter()
            .map(|(s, i)| (s.to_string(), *i))
            .collect();
        Self { map }
    }
    fn get_address(&self, s: &str) -> usize {
        *self.map.get(s).unwrap()
    }
}

pub struct Assembler {
    parser: Parser,
    table: SymbolTable,
}

impl Assembler {
    pub fn new(source: &str) -> Self {
        Assembler {
            parser: Parser::new(source),
            table: SymbolTable::new(),
        }
    }

    pub fn write(&mut self, writer: &mut impl io::Write) -> Result<(), io::Error> {
        let mut need_ln = false;
        while self.parser.has_more_lines() {
            self.parser.advance();
            if need_ln {
                writeln!(writer)?;
            }
            match self.parser.instruction_type() {
                InstructionType::InstA => {
                    let sym = self.parser.symbol();
                    let addr = u16::from_str_radix(sym, 10).unwrap();
                    write!(writer, "{:016b}", addr)?;
                }
                InstructionType::InstC => {
                    let comp = self.parser.comp();
                    let dest = self.parser.dest();
                    let jump = self.parser.jump();

                    write!(
                        writer,
                        "111{}{}{}",
                        code::comp(comp).unwrap(),
                        code::dest(dest).unwrap(),
                        code::jump(jump).unwrap()
                    )?;
                }
                InstructionType::InstL => todo!(),
            }
            need_ln = true;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let source = include_str!("../asm/Add.asm");
        let expected = include_str!("../asm/Add.hack");

        let mut asm = Assembler::new(source);

        let mut buf = Vec::new();
        asm.write(&mut buf).unwrap();

        assert_eq!(expected, String::from_utf8(buf).unwrap())
    }
}
