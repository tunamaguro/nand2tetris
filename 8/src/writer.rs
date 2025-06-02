use super::{ArithmeticCommand, PushPop, PushPopCommand, Segment};

pub struct CodeWriter<W> {
    output: W,
    ident: String,
    jmp_count: u16,
}

impl<W: std::io::Write> CodeWriter<W> {
    pub fn new(output: W, ident: String) -> Self {
        CodeWriter {
            output,
            ident,
            jmp_count: 0,
        }
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
                // X がMレジスタに入る
                self.backward_stack()?;
                self.write_line("D=M-D")?; // x - y
                let cnt = self.increment_jmp_count();
                let when_true = format!("EQ_TRUE_{}", cnt);
                let end = format!("EQ_END_{}", cnt);

                // Dレジスタの値が0ならばEQ_TRUEにジャンプ
                self.write_line(format!("@{}", when_true))?;
                self.write_line("D;JEQ")?;

                // Dレジスタの値が0でなければDレジスタに0をセット
                self.write_line("D=0")?;
                self.write_line(format!("@{}", end))?;
                self.write_line("0;JMP")?;

                // EQ_TRUEにジャンプした場合の処理(-1は補数で11111111)
                self.write_line(format!("({})", when_true))?;
                self.write_line("D=-1")?;

                // EQ_ENDにジャンプ
                self.write_line(format!("({})", end))?;
                self.set_stack_top()?;
            }
            ArithmeticCommand::Gt => {
                // X がMレジスタに入る
                self.backward_stack()?;
                self.write_line("D=M-D")?; // x - y
                let cnt = self.increment_jmp_count();
                let when_true = format!("GT_TRUE_{}", cnt);
                let end = format!("GT_END_{}", cnt);

                // Dレジスタの値が0より大きいならばGT_TRUEにジャンプ
                self.write_line(format!("@{}", when_true))?;
                self.write_line("D;JGT")?;

                // Dレジスタの値が0以下ならDレジスタに0をセット
                self.write_line("D=0")?;
                self.write_line(format!("@{}", end))?;
                self.write_line("0;JMP")?;

                // GT_TRUEにジャンプした場合の処理(-1は補数で11111111)
                self.write_line(format!("({})", when_true))?;
                self.write_line("D=-1")?;

                // GT_ENDにジャンプ
                self.write_line(format!("({})", end))?;
                self.set_stack_top()?;
            }
            ArithmeticCommand::Lt => {
                // X がMレジスタに入る
                self.backward_stack()?;
                self.write_line("D=M-D")?; // x - y
                let cnt = self.increment_jmp_count();
                let when_true = format!("LT_TRUE_{}", cnt);
                let end = format!("LT_END_{}", cnt);

                // Dレジスタの値が0より小さいならばLT_TRUEにジャンプ
                self.write_line(format!("@{}", when_true))?;
                self.write_line("D;JLT")?;

                // Dレジスタの値が0以上ならDレジスタに0をセット
                self.write_line("D=0")?;
                self.write_line(format!("@{}", end))?;
                self.write_line("0;JMP")?;

                // LT_TRUEにジャンプした場合の処理(-1は補数で11111111)
                self.write_line(format!("({})", when_true))?;
                self.write_line("D=-1")?;

                // LT_ENDにジャンプ
                self.write_line(format!("({})", end))?;
                self.set_stack_top()?;
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
                self.set_segment_addr(&command.segment, command.index)?;
                self.write_line("D=A")?;
                self.write_line("@R13")?;
                self.write_line("M=D")?;

                // Dレジスタにスタックのトップの値を保存
                self.backward_stack()?;
                self.write_line("D=M")?;

                // R13に保存したアドレスにDレジスタの値を書き込む
                self.write_line("@R13")?;
                self.write_line("A=M")?;
                self.write_line("M=D")?;

            }
            PushPop::Push => {
                // Dレジスタにセグメントの値を読み込む
                match command.segment {
                    Segment::Constant => {
                        self.write_line(format!("@{}", command.index))?;
                        self.write_line("D=A")?;
                    }
                    _ => {
                        self.set_segment_addr(&command.segment, command.index)?;
                        self.write_line("D=M")?;
                    }
                }
                
                self.set_stack_top()?;
                self.write_line("M=D")?;
                self.advance_stack()?;
            }
        };

        Ok(())
    }

    /// 指定されたセグメントを指すようにAレジスタを設定する
    fn set_segment_addr(&mut self, segment: &Segment, index: u16) -> std::io::Result<()> {
        let addr = match segment {
            Segment::Local => format!("@LCL"),
            Segment::Argument => format!("@ARG"),
            Segment::This => format!("@THIS"),
            Segment::That => format!("@THAT"),
            Segment::Constant => unreachable!("Constant segment can not be used for address"),
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
        };

        self.write_line(addr)?;

        // local, argument, this, that の場合はindexの分だけずらす
        match segment {
            Segment::Local | Segment::Argument | Segment::This | Segment::That => {
                self.write_line("D=M")?;
                self.write_line(format!("@{}", index))?;
                self.write_line("A=D+A")?;
            }
            _ => {}
        }

        // Aレジスタがセグメントを指す
        Ok(())
    }

    /// スタックのトップをAレジスタに設定する
    fn set_stack_top(&mut self) -> std::io::Result<()> {
        self.write_line("@SP")?;
        self.write_line("A=M")?;
        Ok(())
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

    fn increment_jmp_count(&mut self) -> u16 {
        let count = self.jmp_count;
        self.jmp_count += 1;
        count
    }

    fn write_line<S: AsRef<str>>(&mut self, code: S) -> std::io::Result<()> {
        writeln!(self.output, "{}", code.as_ref())
    }
}
