use super::{ArithmeticCommand, Command, PushPop, PushPopCommand, Segment};

pub struct Parser {
    source: String,
    cur_pos: usize,
}

impl Parser {
    pub fn new<R: std::io::Read>(reader: &mut R) -> Self {
        let mut source = String::new();
        reader.read_to_string(&mut source).unwrap();
        Parser { source, cur_pos: 0 }
    }

    pub fn has_more_lines(&self) -> bool {
        self.cur_pos < self.source.len()
    }

    pub(crate) fn advance(&mut self) -> Option<Command> {
        let line = self.source[self.cur_pos..].lines().next()?.trim();
        let cmd = Command::parse(line);
        self.cur_pos += line.len() + 1; // Move past the line and newline character
        cmd
    }
}

pub trait Parsable: Sized {
    type Output;
    fn parse(line: &str) -> Option<Self::Output>;
}

impl Parsable for Command {
    type Output = Self;
    fn parse(line: &str) -> Option<Self::Output> {
        PushPopCommand::parse(line)
            .map(Command::PushPop)
            .or_else(|| ArithmeticCommand::parse(line).map(Command::Arithmetic))
            .or_else(|| parse_function(line))
            .or_else(|| parse_call(line))
            .or_else(|| parse_return(line))
            .or_else(|| parse_jump(line))
    }
}

fn parse_jump(line: &str) -> Option<Command> {
    let line = line.trim();
    let mut parts = line.split_whitespace();

    let tag = parts.next()?;
    let second_part = parts.next()?;
    if parts.next().map(is_not_comment).unwrap_or(false) {
        return None; // Ensure no extra parts are present
    }

    match tag {
        "label" => Some(Command::Label(second_part.to_string())),
        "goto" => Some(Command::GoTo(second_part.to_string())),
        "if-goto" => Some(Command::IfGoTo(second_part.to_string())),
        _ => None,
    }
}

fn parse_function(line: &str) -> Option<Command> {
    let mut parts = line.trim().split_whitespace();
    let func_tag = parts.next()?;
    if func_tag != "function" {
        return None; // Ensure the command is a function declaration
    }

    let name = parts.next()?;
    let n_args = parts.next()?.parse::<u16>().ok()?;
    if parts.next().map(is_not_comment).unwrap_or(false) {
        return None; // Ensure no extra parts are present
    }
    Some(Command::Function {
        name: name.to_string(),
        n_args,
    })
}

fn parse_call(line: &str) -> Option<Command> {
    let mut parts = line.trim().split_whitespace();
    let call_tag = parts.next()?;
    if call_tag != "call" {
        return None; // Ensure the command is a call declaration
    }

    let name = parts.next()?;
    let n_args = parts.next()?.parse::<u16>().ok()?;
    if parts.next().map(is_not_comment).unwrap_or(false) {
        return None; // Ensure no extra parts are present
    }
    Some(Command::Call {
        name: name.to_string(),
        n_args,
    })
}

fn parse_return(line: &str) -> Option<Command> {
    let mut parts = line.trim().split_whitespace();
    let return_tag = parts.next()?;
    if return_tag != "return" {
        return None; // Ensure the command is a return declaration
    }
    if parts.next().map(is_not_comment).unwrap_or(false) {
        return None; // Ensure no extra parts are present
    }

    Some(Command::Return)
}

fn parse_comment(line: &str) -> Option<&str> {
    if let Some((_first, rest)) = line.trim().split_once("//") {
        Some(rest)
    } else {
        None
    }
}

fn is_not_comment(line: &str) -> bool {
    !line.trim().is_empty() && parse_comment(line).is_none()
}

impl Parsable for ArithmeticCommand {
    type Output = Self;
    fn parse(line: &str) -> Option<Self::Output> {
        let mut parts = line.trim().split_whitespace();
        let command = parts.next()?;
        if parts.next().map(is_not_comment).unwrap_or(false) {
            return None; // Ensure no extra parts are present
        }
        match command {
            "add" => Some(ArithmeticCommand::Add),
            "sub" => Some(ArithmeticCommand::Sub),
            "neg" => Some(ArithmeticCommand::Neg),
            "eq" => Some(ArithmeticCommand::Eq),
            "gt" => Some(ArithmeticCommand::Gt),
            "lt" => Some(ArithmeticCommand::Lt),
            "and" => Some(ArithmeticCommand::And),
            "or" => Some(ArithmeticCommand::Or),
            "not" => Some(ArithmeticCommand::Not),
            _ => None,
        }
    }
}

impl Parsable for PushPopCommand {
    type Output = Self;
    fn parse(line: &str) -> Option<Self::Output> {
        let mut parts = line.trim().split_whitespace();

        let kind = parts.next().and_then(|s| PushPop::parse(s))?;
        let segment = parts.next().and_then(|s| Segment::parse(s))?;
        let index = parts.next().and_then(|s| s.parse::<u16>().ok())?;
        if parts.next().map(is_not_comment).unwrap_or(false) {
            return None; // Ensure no extra parts are present
        }

        Some(PushPopCommand {
            kind,
            segment,
            index,
        })
    }
}

impl Parsable for Segment {
    type Output = Self;
    fn parse(line: &str) -> Option<Self::Output> {
        match line.trim() {
            "argument" => Some(Segment::Argument),
            "local" => Some(Segment::Local),
            "static" => Some(Segment::Static),
            "constant" => Some(Segment::Constant),
            "this" => Some(Segment::This),
            "that" => Some(Segment::That),
            "pointer" => Some(Segment::Pointer),
            "temp" => Some(Segment::Temp),
            _ => None,
        }
    }
}

impl Parsable for PushPop {
    type Output = Self;
    fn parse(line: &str) -> Option<Self::Output> {
        match line.trim() {
            "push" => Some(PushPop::Push),
            "pop" => Some(PushPop::Pop),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_push_pop_command() {
        let command = PushPopCommand::parse("push local 2");
        assert!(command.is_some());
        let command = command.unwrap();
        assert_eq!(command.kind, PushPop::Push);
        assert_eq!(command.segment, Segment::Local);
        assert_eq!(command.index, 2);
    }

    #[test]
    fn test_parse_arithmetic_command() {
        let command = ArithmeticCommand::parse("add");
        assert!(command.is_some());
        assert_eq!(command.unwrap(), ArithmeticCommand::Add);
    }

    #[test]
    fn test_parse_invalid_command() {
        let command = Command::parse("invalid command");
        assert!(command.is_none());
    }

    #[test]
    fn test_parse_all_arithmetic_commands() {
        let cmds = [
            ("add", ArithmeticCommand::Add),
            ("sub", ArithmeticCommand::Sub),
            ("neg", ArithmeticCommand::Neg),
            ("eq", ArithmeticCommand::Eq),
            ("gt", ArithmeticCommand::Gt),
            ("lt", ArithmeticCommand::Lt),
            ("and", ArithmeticCommand::And),
            ("or", ArithmeticCommand::Or),
            ("not", ArithmeticCommand::Not),
        ];
        for (text, expected) in cmds.iter() {
            let parsed = ArithmeticCommand::parse(text);
            assert_eq!(parsed, Some(expected.clone()));
        }
    }

    #[test]
    fn test_parse_push_pop_all_segments() {
        let segments = [
            ("argument", Segment::Argument),
            ("local", Segment::Local),
            ("static", Segment::Static),
            ("constant", Segment::Constant),
            ("this", Segment::This),
            ("that", Segment::That),
            ("pointer", Segment::Pointer),
            ("temp", Segment::Temp),
        ];
        for (seg_str, seg_enum) in segments.iter() {
            let push = format!("push {} 7", seg_str);
            let pop = format!("pop {} 3", seg_str);
            let push_cmd = PushPopCommand::parse(&push).unwrap();
            assert_eq!(push_cmd.kind, PushPop::Push);
            assert_eq!(push_cmd.segment, *seg_enum);
            assert_eq!(push_cmd.index, 7);

            let pop_cmd = PushPopCommand::parse(&pop).unwrap();
            assert_eq!(pop_cmd.kind, PushPop::Pop);
            assert_eq!(pop_cmd.segment, *seg_enum);
            assert_eq!(pop_cmd.index, 3);
        }
    }

    #[test]
    fn test_parse_push_pop_invalid() {
        // Invalid kind
        assert!(PushPopCommand::parse("pussh local 2").is_none());
        // Invalid segment
        assert!(PushPopCommand::parse("push foo 2").is_none());
        // Invalid index
        assert!(PushPopCommand::parse("push local x").is_none());
        // Extra argument
        assert!(PushPopCommand::parse("push local 2 extra").is_none());
        // Missing argument
        assert!(PushPopCommand::parse("push local").is_none());
    }

    #[test]
    fn test_command_parse_push_pop_and_arithmetic() {
        let c = Command::parse("push argument 5");
        match c {
            Some(Command::PushPop(cmd)) => {
                assert_eq!(cmd.kind, PushPop::Push);
                assert_eq!(cmd.segment, Segment::Argument);
                assert_eq!(cmd.index, 5);
            }
            _ => panic!("Expected PushPop command"),
        }

        let c = Command::parse("add");
        match c {
            Some(Command::Arithmetic(ArithmeticCommand::Add)) => {}
            _ => panic!("Expected Arithmetic Add command"),
        }
    }
}
