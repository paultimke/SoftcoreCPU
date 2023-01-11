start: 
    mov r1 #25

loop:
    cmp r1 #30
    beq end_loop
    add r1 r1 #1
    b loop

end_loop:
    mov r7 #1

///// HAND DISASSEMBLY
/*
ADDRESS |      BINARY         | HEX
0:        0000 0001 0001 1001   0x0119  <- start    | mov r1 #25
1:        1010 0001 0001 1110   0xA11E  <- loop     | cmp r1 #30
2:        0100 0001 0010 0001   0x4121              | add r1 r1 #1
3:        1011 1000 0000 0001   0xB801              | bne loop
4:        0000 0111 0000 0001   0x0701  <- end_loop | mov r7 #1
5:        1110 0000 0000 0000   0xE000              | halt
6:        0000 0000 0000 0000   0x0000
*/
