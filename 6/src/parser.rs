use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionType {
    /// @xxx
    InstA,
    /// dest=xxx;jmp
    InstC,
    /// (xxx)
    InstL,
}

#[derive(Debug, Clone)]
pub struct Parser {
    source: String,
    pos: RefCell<usize>,
    // has called advance()?
    has_advance: bool,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            pos: RefCell::new(0),
            has_advance: false,
        }
    }

    pub fn has_more_lines(&self) -> bool {
        let s = self.get_rest();
        let skip = if self.has_advance { 1 } else { 0 };
        for line in s.split("\n").skip(skip).map(remove_head_space) {
            if !is_comment(s) && !line.is_empty() {
                return true;
            }
        }
        false
    }

    pub fn advance(&mut self) {
        loop {
            let line = self.peek_line();
            let trimmed = remove_head_space(line);
            if is_comment(trimmed) || trimmed.is_empty() {
                *self.pos.borrow_mut() += line.len() + 1;
                continue;
            }
            if self.has_advance {
                *self.pos.borrow_mut() += line.len() + 1;
            }

            let next_line = remove_head_space(self.peek_line());
            if !is_comment(next_line) && !next_line.is_empty() {
                break;
            }
        }

        self.has_advance = true;
    }

    pub fn instruction_type(&self) -> InstructionType {
        let l = self.peek_line();

        if l.starts_with("@") {
            InstructionType::InstA
        } else if l.starts_with("(") && l.ends_with(")") {
            InstructionType::InstL
        } else {
            InstructionType::InstC
        }
    }

    pub fn symbol(&self) -> &str {
        let line = self.peek_line();
        let inst = self.instruction_type();
        match inst {
            InstructionType::InstA => {
                // skip first `@`
                &line[1..]
            }
            InstructionType::InstC => panic!("expected a instruction or label"),
            InstructionType::InstL => {
                let size = line.len();
                &line[1..(size - 1)]
            }
        }
    }

    pub fn dest(&self) -> &str {
        let line = self.peek_line();
        let inst = self.instruction_type();
        match inst {
            InstructionType::InstC => {
                if let Some(p) = line.find("=") {
                    &line[..p]
                } else {
                    ""
                }
            }
            _ => panic!("expected c instruction"),
        }
    }

    pub fn comp(&self) -> &str {
        let line = self.peek_line();
        let inst = self.instruction_type();
        match inst {
            InstructionType::InstC => {
                let mut start = 0;
                let mut end = line.len();
                if let Some(sp) = line.find("=") {
                    start = sp + 1;
                };
                if let Some(ep) = line.find(";") {
                    end = ep;
                };
                &line[start..end]
            }
            _ => panic!("expected c instruction"),
        }
    }

    pub fn jump(&self) -> &str {
        let line = self.peek_line();
        let inst = self.instruction_type();
        match inst {
            InstructionType::InstC => {
                if let Some(p) = line.find(";") {
                    &line[(p + 1)..]
                } else {
                    ""
                }
            }
            _ => panic!("expected c instruction"),
        }
    }

    fn get_rest(&self) -> &str {
        let p = *self.pos.borrow();
        &self.source[p..]
    }

    fn peek_line(&self) -> &str {
        let s = self.get_rest();
        let l = if let Some(end) = s.find("\n") {
            &s[..end]
        } else {
            s
        };
        remove_head_space(l)
    }
}

fn remove_head_space(s: &str) -> &str {
    s.trim_start_matches(" ")
}

fn is_comment(s: &str) -> bool {
    s.starts_with("//")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dst_comp_c_instruction() {
        {
            let parser = Parser::new("D=A");

            assert_eq!(parser.dest(), "D");
            assert_eq!(parser.comp(), "A");
            assert_eq!(parser.jump(), "");
        }

        {
            let parser = Parser::new("D=D+A");

            assert_eq!(parser.dest(), "D");
            assert_eq!(parser.comp(), "D+A");
            assert_eq!(parser.jump(), "");
        }
    }

    #[test]
    fn test_parse_comp_jump_c_instruction() {
        {
            let parser = Parser::new("0;JMP");

            assert_eq!(parser.dest(), "");
            assert_eq!(parser.comp(), "0");
            assert_eq!(parser.jump(), "JMP");
        }

        {
            let parser = Parser::new("D;JMP");

            assert_eq!(parser.dest(), "");
            assert_eq!(parser.comp(), "D");
            assert_eq!(parser.jump(), "JMP");
        }
    }

    #[test]
    fn test_parse_dest_comp_jump_c_instruction() {
        {
            let parser = Parser::new("D=1;JMP");

            assert_eq!(parser.dest(), "D");
            assert_eq!(parser.comp(), "1");
            assert_eq!(parser.jump(), "JMP");
        }

        {
            let parser = Parser::new("M=D;JNE");

            assert_eq!(parser.dest(), "M");
            assert_eq!(parser.comp(), "D");
            assert_eq!(parser.jump(), "JNE");
        }
    }

    #[test]
    fn test_parse_a_instruction() {
        {
            let parser = Parser::new("@123");

            assert_eq!(parser.symbol(), "123")
        }

        {
            let parser = Parser::new("@xxx");

            assert_eq!(parser.symbol(), "xxx")
        }

        {
            let parser = Parser::new("@xxx   ");

            assert_eq!(parser.symbol(), "xxx   ")
        }
    }

    #[test]
    fn test_parse_label() {
        {
            let parser = Parser::new("(abc)");

            assert_eq!(parser.symbol(), "abc")
        }

        {
            let parser = Parser::new("(LOOP  )");

            assert_eq!(parser.symbol(), "LOOP  ")
        }
    }

    #[test]
    fn test_advance() {
        let source = r#"
// some comment


@R0
D=M
(LABEL)
  // comment
  @R0
        "#;

        let mut parser = Parser::new(source);

        // move to first instruction
        assert!(parser.has_more_lines());
        parser.advance();
        assert_eq!(parser.instruction_type(), InstructionType::InstA);

        // next instruction
        assert!(parser.has_more_lines());
        parser.advance();
        assert_eq!(parser.instruction_type(), InstructionType::InstC);

        // next instruction
        assert!(parser.has_more_lines());
        parser.advance();
        assert_eq!(parser.instruction_type(), InstructionType::InstL);

        // next instruction
        assert!(parser.has_more_lines());
        parser.advance();
        assert_eq!(parser.instruction_type(), InstructionType::InstA);

        // no instruction
        assert_eq!(parser.has_more_lines(), false);
    }
}
