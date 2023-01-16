// Test File 1: Sum elements of an array
.section[code]
start: 
    mov r1 #0  // Loop counter
    mov r3 #0  // Sum variable <- 0
    lda arr    // Load start of array in MBR (r7)
    mov r2 mbr // Move to r2 to use as array start pointer

loop: 
    cmp r1 #5
    beq end_loop
    ldr r4 &r2    // r4 holds arr[r2]
    add r3 r3 r4  // r3 + arr[2]
    add r2 r2 #1  // Increment array pointer
    add r1 r1 #1  // Increment loop counter
    jmp loop

end_loop:
    mov r0 r3
    halt

.section[data]
arr: 
    17, 22, -1, 4, 38

///// HAND DISASSEMBLY
//
// START SECTION CODE
// ADDRESS |      BINARY         | HEX
// 0:        0000 0001 0000 0000   0x0100  <- start    | mov r1 #0
// 1:        0000 0011 0000 0000   0x0300              | mov r3 #0
// 2:        0001 0000 0000 1101   0x100D              | lda &arr
// 3:        0000 1010 1110 0000   0x0AE0              | mov r2 mbr
// 4:        1010 0001 0000 0101   0xA105  <- loop     | cmp r1 #5
// 5:        1011 0000 0000 1011   0xB00B              | beq end_loop
// 6:        0001 1100 0100 0000   0x1C40              | ldr r4 r2
// 7:        0100 1011 0111 0000   0x4B70              | add r3 r3 r4 
// 8:        0100 0010 0100 0001   0x4241              | add r2 r2 #1
// 9:        0100 0001 0010 0001   0x4121              | add r1 r1 #1 *fail
// 10:       1000 1000 0000 0100   0x8804              | jmp loop
// 11:       0000 1000 0110 0000   0x0860  <- end_loop | mov r0 r3 *fail
// 12:       1110 0000 0000 0000   0xE000              | halt
// START SECTION DATA
// ADDRESS |      BINARY         | HEX
// 13:       0000 0000 0001 0001   0x0011  <- arr
// 14:       0000 0000 0001 0110   0x0016
// 15:       1111 1111 1111 1111   0xFFFF
// 16:       0000 0000 0000 0100   0x0004
// 17:       0000 0000 0010 0110   0x0026