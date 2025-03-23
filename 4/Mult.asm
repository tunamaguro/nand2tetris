@i
M=0
@result
M=0

(LOOP)
// if i > R0 goto STOP
@i
D=M
@R0
D=D-M
@STOP
D;JEQ

// i = i + 1
@i
M=M+1

// result = result + R1
@R1
D=M
@result
M=D+M

@LOOP
0;JMP

(STOP)
// R2 = result
@result
D=M
@R2
M=D

(END)
@END
0;JMP