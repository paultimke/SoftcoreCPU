// TEST FILE 2: TESTING SECTION DECLARATIONS
//.section(data)
//array:
//    30, 20, -2, 4

.section[code]
start:
    mov r1 #7
    mov r2 r1
//    jmp end
//
//end:
//    add r1 r1 #7

.section(data)
array:
    30, 20, -2, 4

// END OF FILE
// Code first:
//     code(start, mid) data(mid, end)
// Data Fist:
//     code(mid, end) data(start, mid)
// Code, NO Data:
//     code(start, 0) data(0, 0)
// Data, No Code:
//     code(0, 0) data(start, 0)
// Neither:
//     code(0, 0) data(0, 0)