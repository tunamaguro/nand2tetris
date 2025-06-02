use super::{ArithmeticCommand, PushPop, PushPopCommand, Segment};

pub struct CodeWriter<W> {
    output: W,
    ident: String,
    function_names: Vec<String>,
    jmp_count: u16,
}

impl<W: std::io::Write> CodeWriter<W> {
    pub fn new(output: W, ident: String) -> Self {
        CodeWriter {
            output,
            ident,
            function_names: Vec::new(),
            jmp_count: 0,
        }
    }

    pub fn set_ident(&mut self, ident: String) {
        self.ident = ident;
        self.jmp_count = 0; // Reset jump count when identifier changes
    }

    fn function_name(&self) -> String {
        if let Some(name) = self.function_names.last() {
            format!("{}.{}", self.ident, name)
        } else {
            self.ident.clone()
        }
    }

    pub fn init(&mut self) -> std::io::Result<()> {
        // スタックポインタを初期化
        writeln!(self.output, "@256")?;
        writeln!(self.output, "D=A")?;
        writeln!(self.output, "@SP")?;
        writeln!(self.output, "M=D")?;
        Ok(())
    }

    pub fn finalize(&mut self) -> std::io::Result<()> {
        // プログラム終了のためのコード
        let tag = format!("{}.END", self.ident);
        writeln!(self.output, "({})", tag)?;
        writeln!(self.output, "@{}", tag)?;
        writeln!(self.output, "0;JMP")?;
        Ok(())
    }

    pub(crate) fn write_arithmetic(&mut self, command: &ArithmeticCommand) -> std::io::Result<()> {
        // Y をDレジスタに保持
        self.backward_stack()?;
        writeln!(self.output, "D=M")?;

        // どうにかして計算結果をDレジスタに入れる
        // Aレジスタにはスタックのトップが入っているようにする
        match command {
            ArithmeticCommand::Add => {
                // X がMレジスタに入る
                self.backward_stack()?;
                writeln!(self.output, "D=D+M")?;
            }
            ArithmeticCommand::Sub => {
                // X がMレジスタに入る
                self.backward_stack()?;
                writeln!(self.output, "D=M-D")?;
            }
            ArithmeticCommand::Eq => {
                // X がMレジスタに入る
                self.backward_stack()?;
                writeln!(self.output, "D=M-D")?; // x - y
                let cnt = self.increment_jmp_count();
                let when_true = format!("EQ_TRUE_{}", cnt);
                let end = format!("EQ_END_{}", cnt);

                // Dレジスタの値が0ならばEQ_TRUEにジャンプ
                writeln!(self.output, "@{}", when_true)?;
                writeln!(self.output, "D;JEQ")?;

                // Dレジスタの値が0でなければDレジスタに0をセット
                writeln!(self.output, "D=0")?;
                writeln!(self.output, "@{}", end)?;
                writeln!(self.output, "0;JMP")?;

                // EQ_TRUEにジャンプした場合の処理(-1は補数で11111111)
                writeln!(self.output, "{}", format!("({})", when_true))?;
                writeln!(self.output, "D=-1")?;

                // EQ_ENDにジャンプ
                writeln!(self.output, "({})", end)?;
                self.set_stack_top()?;
            }
            ArithmeticCommand::Gt => {
                // X がMレジスタに入る
                self.backward_stack()?;
                writeln!(self.output, "D=M-D")?; // x - y
                let cnt = self.increment_jmp_count();
                let when_true = format!("GT_TRUE_{}", cnt);
                let end = format!("GT_END_{}", cnt);

                // Dレジスタの値が0より大きいならばGT_TRUEにジャンプ
                writeln!(self.output, "@{}", when_true)?;
                writeln!(self.output, "D;JGT")?;

                // Dレジスタの値が0以下ならDレジスタに0をセット
                writeln!(self.output, "D=0")?;
                writeln!(self.output, "@{}", end)?;
                writeln!(self.output, "0;JMP")?;

                // GT_TRUEにジャンプした場合の処理(-1は補数で11111111)
                writeln!(self.output, "({})", when_true)?;
                writeln!(self.output, "D=-1")?;

                // GT_ENDにジャンプ
                writeln!(self.output, "({})", end)?;
                self.set_stack_top()?;
            }
            ArithmeticCommand::Lt => {
                // X がMレジスタに入る
                self.backward_stack()?;
                writeln!(self.output, "D=M-D")?; // x - y
                let cnt = self.increment_jmp_count();
                let when_true = format!("LT_TRUE_{}", cnt);
                let end = format!("LT_END_{}", cnt);

                // Dレジスタの値が0より小さいならばLT_TRUEにジャンプ
                writeln!(self.output, "@{}", when_true)?;
                writeln!(self.output, "D;JLT")?;

                // Dレジスタの値が0以上ならDレジスタに0をセット
                writeln!(self.output, "D=0")?;
                writeln!(self.output, "@{}", end)?;
                writeln!(self.output, "0;JMP")?;

                // LT_TRUEにジャンプした場合の処理(-1は補数で11111111)
                writeln!(self.output, "({})", when_true)?;
                writeln!(self.output, "D=-1")?;

                // LT_ENDにジャンプ
                writeln!(self.output, "({})", end)?;
                self.set_stack_top()?;
            }
            ArithmeticCommand::Neg => {
                writeln!(self.output, "D=-M")?;
            }
            ArithmeticCommand::And => {
                // X がMレジスタに入る
                self.backward_stack()?;
                writeln!(self.output, "D=D&M")?;
            }
            ArithmeticCommand::Or => {
                // X がMレジスタに入る
                self.backward_stack()?;
                writeln!(self.output, "D=D|M")?;
            }
            ArithmeticCommand::Not => {
                writeln!(self.output, "D=!M")?;
            }
        }

        writeln!(self.output, "M=D")?;
        self.advance_stack()?;
        Ok(())
    }

    pub(crate) fn write_push_pop(&mut self, command: &PushPopCommand) -> std::io::Result<()> {
        match command.kind {
            PushPop::Pop => {
                // セグメントの書き込み先アドレスをR13に保存
                self.set_segment_addr(&command.segment, command.index)?;
                writeln!(self.output, "D=A")?;
                writeln!(self.output, "@R13")?;
                writeln!(self.output, "M=D")?;

                // Dレジスタにスタックのトップの値を保存
                self.backward_stack()?;
                writeln!(self.output, "D=M")?;

                // R13に保存したアドレスにDレジスタの値を書き込む
                writeln!(self.output, "@R13")?;
                writeln!(self.output, "A=M")?;
                writeln!(self.output, "M=D")?;
            }
            PushPop::Push => {
                // Dレジスタにセグメントの値を読み込む
                match command.segment {
                    Segment::Constant => {
                        writeln!(self.output, "@{}", command.index)?;
                        writeln!(self.output, "D=A")?;
                    }
                    _ => {
                        self.set_segment_addr(&command.segment, command.index)?;
                        writeln!(self.output, "D=M")?;
                    }
                }

                self.set_stack_top()?;
                writeln!(self.output, "M=D")?;
                self.advance_stack()?;
            }
        };

        Ok(())
    }

    pub(crate) fn write_label(&mut self, label: &str) -> std::io::Result<()> {
        writeln!(self.output, "({}.{})", self.function_name(), label)?;
        Ok(())
    }

    pub(crate) fn write_goto(&mut self, label: &str) -> std::io::Result<()> {
        writeln!(self.output, "@{}.{}", self.function_name(), label)?;
        writeln!(self.output, "0;JMP")?;
        Ok(())
    }

    pub(crate) fn write_if_goto(&mut self, label: &str) -> std::io::Result<()> {
        self.backward_stack()?;
        writeln!(self.output, "D=M")?;
        writeln!(self.output, "@{}.{}", self.function_name(), label)?;
        writeln!(self.output, "D;JNE")?;
        Ok(())
    }

    pub(crate) fn write_function(&mut self, name: &str, n_args: u16) -> std::io::Result<()> {
        todo!()
    }

    pub(crate) fn write_call(&mut self, name: &str, n_args: u16) -> std::io::Result<()> {
        todo!()
    }

    pub(crate) fn write_return(&mut self) -> std::io::Result<()> {
        todo!()
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

        writeln!(self.output, "{}", addr)?;

        // local, argument, this, that の場合はindexの分だけずらす
        match segment {
            Segment::Local | Segment::Argument | Segment::This | Segment::That => {
                writeln!(self.output, "D=M")?;
                writeln!(self.output, "@{}", index)?;
                writeln!(self.output, "A=D+A")?;
            }
            _ => {}
        }

        // Aレジスタがセグメントを指す
        Ok(())
    }

    /// スタックのトップをAレジスタに設定する
    fn set_stack_top(&mut self) -> std::io::Result<()> {
        writeln!(self.output, "@SP")?;
        writeln!(self.output, "A=M")?;
        Ok(())
    }

    /// スタックポインタを1増やし、Aレジスタをスタックのトップに設定する
    fn advance_stack(&mut self) -> std::io::Result<()> {
        writeln!(self.output, "@SP")?;
        writeln!(self.output, "M=M+1")?;
        writeln!(self.output, "A=M")?;
        Ok(())
    }

    /// スタックポインタを1減らし、Aレジスタをスタックのトップに設定する
    fn backward_stack(&mut self) -> std::io::Result<()> {
        writeln!(self.output, "@SP")?;
        writeln!(self.output, "M=M-1")?;
        writeln!(self.output, "A=M")?;
        Ok(())
    }

    fn increment_jmp_count(&mut self) -> u16 {
        let count = self.jmp_count;
        self.jmp_count += 1;
        count
    }
}
