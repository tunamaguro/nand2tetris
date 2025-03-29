use std::io;

mod code;
mod parser;

use parser::{InstructionType, Parser};

#[derive(Debug, Clone)]
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
    fn add_entry(&mut self, key: &str, addr: usize) {
        let _ = self.map.insert(key.to_string(), addr);
    }

    fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    fn get_address(&self, s: &str) -> Option<usize> {
        self.map.get(s).map(|v| *v)
    }
}

pub struct Assembler {
    parser: Parser,
    table: SymbolTable,
}

impl Assembler {
    const RAM_ADDR_START: usize = 16;
    pub fn new(source: &str) -> Self {
        Assembler {
            parser: Parser::new(source),
            table: SymbolTable::new(),
        }
    }

    pub fn write(&mut self, writer: &mut impl io::Write) -> Result<(), io::Error> {
        self.first_path();
        self.second_path(writer)
    }

    fn first_path(&mut self) {
        while self.parser.has_more_lines() {
            self.parser.advance();
            match self.parser.instruction_type() {
                InstructionType::InstA => {}
                InstructionType::InstL => {
                    let sym = self.parser.symbol();
                    let row = self.parser.row();
                    if !self.table.contains(sym) {
                        self.table.add_entry(sym, row);
                    }
                }
                InstructionType::InstC => {}
            }
        }

        self.parser.reset();
    }

    fn second_path(&mut self, writer: &mut impl io::Write) -> Result<(), io::Error> {
        let mut need_ln = false;
        let mut a_count = 0;
        while self.parser.has_more_lines() {
            self.parser.advance();
            match self.parser.instruction_type() {
                InstructionType::InstA => {
                    if need_ln {
                        writeln!(writer)?;
                    }
                    let sym = self.parser.symbol();
                    if let Ok(addr) = u16::from_str_radix(sym, 10) {
                        write!(writer, "{:016b}", addr)?;
                    } else {
                        // use label
                        if let Some(addr) = self.table.get_address(sym) {
                            let addr = u16::try_from(addr).unwrap();
                            write!(writer, "{:016b}", addr)?;
                        } else {
                            let addr = Self::RAM_ADDR_START + a_count;
                            self.table.add_entry(sym, addr);
                            a_count += 1;
                            write!(writer, "{:016b}", addr)?;
                        }
                    };
                }
                InstructionType::InstC => {
                    if need_ln {
                        writeln!(writer)?;
                    }
                    // dbg!(self.parser.peek_line());
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
                InstructionType::InstL => {
                    // do nothing
                }
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

    #[test]
    fn test_max() {
        let source = include_str!("../asm/Max.asm");
        let expected = include_str!("../asm/Max.hack");

        let mut asm = Assembler::new(source);

        let mut buf = Vec::new();
        asm.write(&mut buf).unwrap();

        assert_eq!(expected, String::from_utf8(buf).unwrap())
    }

    #[test]
    fn test_max_l() {
        let source = include_str!("../asm/MaxL.asm");
        let expected = include_str!("../asm/MaxL.hack");

        let mut asm = Assembler::new(source);

        let mut buf = Vec::new();
        asm.write(&mut buf).unwrap();

        assert_eq!(expected, String::from_utf8(buf).unwrap())
    }

    #[test]
    fn test_rect() {
        let source = include_str!("../asm/Rect.asm");
        let expected = include_str!("../asm/Rect.hack");

        let mut asm = Assembler::new(source);

        let mut buf = Vec::new();
        asm.write(&mut buf).unwrap();

        assert_eq!(expected, String::from_utf8(buf).unwrap())
    }

    #[test]
    fn test_rect_l() {
        let source = include_str!("../asm/RectL.asm");
        let expected = include_str!("../asm/RectL.hack");

        let mut asm = Assembler::new(source);

        let mut buf = Vec::new();
        asm.write(&mut buf).unwrap();

        assert_eq!(expected, String::from_utf8(buf).unwrap())
    }

    #[test]
    fn test_pong() {
        let source = include_str!("../asm/Pong.asm");
        let expected = include_str!("../asm/Pong.hack");

        let mut asm = Assembler::new(source);

        let mut buf = Vec::new();
        asm.write(&mut buf).unwrap();

        assert_eq!(expected, String::from_utf8(buf).unwrap())
    }

    #[test]
    fn test_pong_l() {
        let source = include_str!("../asm/PongL.asm");
        let expected = include_str!("../asm/PongL.hack");

        let mut asm = Assembler::new(source);

        let mut buf = Vec::new();
        asm.write(&mut buf).unwrap();

        assert_eq!(expected, String::from_utf8(buf).unwrap())
    }
}
