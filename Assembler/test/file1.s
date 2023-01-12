// Test File 1: Sum elements of an array
.section[code]
start: 
    mov r1 #0  // Loop counter
    mov r3 #0  // Sum variable <- 0
    lda &arr   // Load start of array in MBR (r7)
    mov r2 mbr // Move to r2 to use as array start pointer

loop: 
    cmp r1 #5
    beq end_loop
    ldr r4 r2     // r4 holds arr[r2]
    add r3 r4     // r3 + arr[2]
    add r2 r2 #1  // Increment array pointer
    add r0 r1 #1  // Increment loop counter
    jmp loop

end_loop:
    mov r7 #1
    halt

.section[data]
arr: 
    17, 22, -1, 4, 38

///// HAND DISASSEMBLY
//
// ADDRESS |      BINARY         | HEX
// 0:        0000 0001 0001 1001   0x0119  <- start    | mov r1 #25
// 1:        1010 0001 0001 1110   0xA11E  <- loop     | cmp r1 #30
// 2:        1011 0001 0000 0101   0xB105              | beq end_loop
// 3:        0100 0001 0010 0001   0x4121              | add r1 r1 #1
// 4:        1000 1000 0000 0001   0x0701              | jmp loop
// 5:        0000 0111 0000 0001   0x0701  <- end_loop | mov r7 #1
// 6:        1110 0000 0000 0000   0xE000              | halt
// 7:        
//