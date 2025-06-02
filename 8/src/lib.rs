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
                    Command::Label(label) => {
                        self.writer.write_label(&label)?;
                    }
                    Command::GoTo(label) => self.writer.write_goto(&label)?,
                    Command::IfGoTo(label) => self.writer.write_if_goto(&label)?,
                    Command::Function { name, n_args } => {
                        self.writer.write_function(&name, n_args)?
                    }
                    Command::Call { name, n_args } => self.writer.write_call(&name, n_args)?,
                    Command::Return => self.writer.write_return()?,
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
    Label(String),
    GoTo(String),
    IfGoTo(String),
    Function { name: String, n_args: u16 },
    Call { name: String, n_args: u16 },
    Return,
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
