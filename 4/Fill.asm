// MAXCNT = 32 x 256 = 8192
@8192
D=A
@MAXCNT
M=D

(LOOP)

@COLOR
M=0

// if kbd == 0 skip black
@KBD
D=M
@FILL_START
D;JEQ
(BLACK)
@COLOR
M=-1

(FILL_START)

@SCREEN
D=A
@i
M=D

(FILL_LOOP)
@COLOR
D=M
@i
A=M
M=D
A=A+1
D=A
@i
M=D

// if i == SCREEN + 8192 goto LOOP
@SCREEN
D=D-A
@MAXCNT
D=D-M
@LOOP
D;JEQ

@FILL_LOOP
0;JMP
