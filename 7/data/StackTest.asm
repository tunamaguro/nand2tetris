@256
D=A
@SP
M=D
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@EQ_TRUE_0
D;JEQ
D=0
@EQ_END_0
0;JMP
(EQ_TRUE_0)
D=-1
(EQ_END_0)
@SP
A=M
M=D
@SP
M=M+1
A=M
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@EQ_TRUE_1
D;JEQ
D=0
@EQ_END_1
0;JMP
(EQ_TRUE_1)
D=-1
(EQ_END_1)
@SP
A=M
M=D
@SP
M=M+1
A=M
@16
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@17
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@EQ_TRUE_2
D;JEQ
D=0
@EQ_END_2
0;JMP
(EQ_TRUE_2)
D=-1
(EQ_END_2)
@SP
A=M
M=D
@SP
M=M+1
A=M
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LT_TRUE_3
D;JLT
D=0
@LT_END_3
0;JMP
(LT_TRUE_3)
D=-1
(LT_END_3)
@SP
A=M
M=D
@SP
M=M+1
A=M
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@892
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LT_TRUE_4
D;JLT
D=0
@LT_END_4
0;JMP
(LT_TRUE_4)
D=-1
(LT_END_4)
@SP
A=M
M=D
@SP
M=M+1
A=M
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@891
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LT_TRUE_5
D;JLT
D=0
@LT_END_5
0;JMP
(LT_TRUE_5)
D=-1
(LT_END_5)
@SP
A=M
M=D
@SP
M=M+1
A=M
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@GT_TRUE_6
D;JGT
D=0
@GT_END_6
0;JMP
(GT_TRUE_6)
D=-1
(GT_END_6)
@SP
A=M
M=D
@SP
M=M+1
A=M
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@GT_TRUE_7
D;JGT
D=0
@GT_END_7
0;JMP
(GT_TRUE_7)
D=-1
(GT_END_7)
@SP
A=M
M=D
@SP
M=M+1
A=M
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@GT_TRUE_8
D;JGT
D=0
@GT_END_8
0;JMP
(GT_TRUE_8)
D=-1
(GT_END_8)
@SP
A=M
M=D
@SP
M=M+1
A=M
@57
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@31
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@53
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D+M
M=D
@SP
M=M+1
A=M
@112
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
D=-M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D&M
M=D
@SP
M=M+1
A=M
@82
D=A
@SP
A=M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=D|M
M=D
@SP
M=M+1
A=M
@SP
M=M-1
A=M
D=M
D=!M
M=D
@SP
M=M+1
A=M
(StackTest.END)
@StackTest.END
0;JMP
