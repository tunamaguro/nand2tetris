use super::{ArithmeticCommand, PushPop, PushPopCommand, Segment};

pub struct CodeWriter<W> {
    output: W,
    ident: String,
}

impl<W: std::io::Write> CodeWriter<W> {
    pub fn new(output: W, ident: String) -> Self {
        CodeWriter { output, ident }
    }

    pub fn init(&mut self) -> std::io::Result<()> {
        // スタックポインタを初期化
        self.write_line("@256")?;
        self.write_line("D=A")?;
        self.write_line("@SP")?;
        self.write_line("M=D")?;
        Ok(())
    }

    pub fn finalize(&mut self) -> std::io::Result<()> {
        // プログラム終了のためのコード
        let tag = format!("{}.END", self.ident);
        self.write_line(format!("({})", tag))?;
        self.write_line(format!("@{}", tag))?;
        self.write_line("0;JMP")?;
        Ok(())
    }

    pub(crate) fn write_arithmetic(&mut self, command: &ArithmeticCommand) -> std::io::Result<()> {
        // Y をDレジスタに保持
        self.backward_stack()?;
        self.write_line("D=M")?;

        // どうにかして計算結果をDレジスタに入れる
        // Aレジスタにはスタックのトップが入っているようにする
        match command {
            ArithmeticCommand::Add => {
                // X がMレジスタに入る
                self.backward_stack()?;
                self.write_line("D=D+M")?;
            }
            ArithmeticCommand::Sub => {
                // X がMレジスタに入る
                self.backward_stack()?;
                self.write_line("D=M-D")?;
            }
            ArithmeticCommand::Eq => {
                self.write_line("// OP EQ")?;
                todo!();
            }
            ArithmeticCommand::Gt => {
                self.write_line("// OP GT")?;
                todo!();
            }
            ArithmeticCommand::Lt => {
                self.write_line("// OP LT")?;
                todo!();
            }
            ArithmeticCommand::Neg => {
                self.write_line("D=-M")?;
            }
            ArithmeticCommand::And => {
                // X がMレジスタに入る
                self.backward_stack()?;
                self.write_line("D=D&M")?;
            }
            ArithmeticCommand::Or => {
                // X がMレジスタに入る
                self.backward_stack()?;
                self.write_line("D=D|M")?;
            }
            ArithmeticCommand::Not => {
                self.write_line("D=!M")?;
            }
        }

        self.write_line("M=D")?;
        self.advance_stack()?;
        Ok(())
    }

    pub(crate) fn write_push_pop(&mut self, command: &PushPopCommand) -> std::io::Result<()> {
        match command.kind {
            PushPop::Pop => {
                // セグメントの書き込み先アドレスをR13に保存
                let addr = self.segment_addr(&command.segment, command.index);
                self.write_line(addr)?;
                // local, argument, this, that の場合はindexの分だけずらす
                match command.segment {
                    Segment::Local | Segment::Argument | Segment::This | Segment::That => {
                        self.write_line("D=A")?;
                        self.write_line(format!("@{}", command.index))?;
                        self.write_line("A=D+A")?;
                    }
                    _ => {}
                }
                self.write_line("D=M")?;
                self.write_line("@R13")?;
                self.write_line("M=D")?;

                // Dレジスタにスタックのトップの値を保存
                self.write_line("@SP")?;
                self.write_line("D=M")?;

                // R13に保存したアドレスにDレジスタの値を書き込む
                self.write_line("@R13")?;
                self.write_line("A=M")?;
                self.write_line("M=D")?;

                self.backward_stack()?;
            }
            PushPop::Push => {
                // Dレジスタにセグメントの値を読み込む
                match command.segment {
                    Segment::Constant => {
                        self.write_line(format!("@{}", command.index))?;
                        self.write_line("D=A")?;
                    }
                    _ => {
                        let addr = self.segment_addr(&command.segment, command.index);
                        self.write_line(addr)?;

                        // local, argument, this, that の場合はindexの分だけずらす
                        match command.segment {
                            Segment::Local | Segment::Argument | Segment::This | Segment::That => {
                                self.write_line("D=A")?;
                                self.write_line(format!("@{}", command.index))?;
                                self.write_line("A=D+A")?;
                            }
                            _ => {}
                        }
                        self.write_line("D=M")?;
                    }
                }
                self.write_line("@SP")?;
                self.write_line("A=M")?;
                self.write_line("M=D")?;
                self.advance_stack()?;
            }
        };

        Ok(())
    }

    fn segment_addr(&self, segment: &Segment, index: u16) -> String {
        match segment {
            Segment::Local => format!("@LCL"),
            Segment::Argument => format!("@ARG"),
            Segment::This => format!("@THIS"),
            Segment::That => format!("@THAT"),
            Segment::Constant => unreachable!("Constant segment should not be used for address"),
            Segment::Static => {
                if index > 240 {
                    panic!("Static segment index out of bounds: {}", index);
                }
                format!("@{}.{}", self.ident, index)
            }
            Segment::Temp => {
                if index > 7 {
                    panic!("Temp segment index out of bounds: {}", index);
                }
                format!("@R{}", 5 + index)
            }
            Segment::Pointer => {
                if index == 0 {
                    "@THIS".to_string()
                } else {
                    "@THAT".to_string()
                }
            }
        }
    }

    /// スタックポインタを1増やし、Aレジスタをスタックのトップに設定する
    fn advance_stack(&mut self) -> std::io::Result<()> {
        self.write_line("@SP")?;
        self.write_line("M=M+1")?;
        self.write_line("A=M")?;
        Ok(())
    }

    /// スタックポインタを1減らし、Aレジスタをスタックのトップに設定する
    fn backward_stack(&mut self) -> std::io::Result<()> {
        self.write_line("@SP")?;
        self.write_line("M=M-1")?;
        self.write_line("A=M")?;
        Ok(())
    }

    fn write_line<S: AsRef<str>>(&mut self, code: S) -> std::io::Result<()> {
        writeln!(self.output, "{}", code.as_ref())
    }
}
