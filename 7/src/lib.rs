mod parser;
mod writer;

pub use parser::Parser;
pub use writer::CodeWriter;

pub struct VmTranslator<W> {
    parser: parser::Parser,
    writer: writer::CodeWriter<W>,
}

impl<W: std::io::Write> VmTranslator<W> {
    pub fn new(parser: Parser, writer: writer::CodeWriter<W>) -> Self {
        VmTranslator { parser, writer }
    }

    pub fn translate(&mut self) -> std::io::Result<()> {
        self.writer.init()?;
        while self.parser.has_more_lines() {
            if let Some(command) = self.parser.advance() {
                match command {
                    Command::PushPop(push_pop_command) => {
                        self.writer.write_push_pop(&push_pop_command)?;
                    }
                    Command::Arithmetic(arithmetic_command) => {
                        self.writer.write_arithmetic(&arithmetic_command)?;
                    }
                }
            }
        }
        self.writer.finalize()?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Command {
    PushPop(PushPopCommand),
    Arithmetic(ArithmeticCommand),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct PushPopCommand {
    kind: PushPop,
    segment: Segment,
    index: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PushPop {
    Push,
    Pop,
}
