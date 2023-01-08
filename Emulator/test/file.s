mov r0, #25
mov r1, #12
add r2, r0, r1
sub r3, r0, r1
shl r1, r1, #2

mov r7, r1 // Moves the value to MBR to store it in RAM
str #0x18  // Stores MBR (r1) into location decimal 24

load #0x18 // Loads value previously stored into MBR
mov r5, r7 // Moves value from MBR into register 5

///// HAND DISASSEMBLY:
/*
ADDRESS |      BINARY         | HEX
0:        0000 0000 0001 1001   0x0019
1:        0000 0001 0000 1100   0x010C
2:        0100 1010 0000 0100   0x4A04
3:        0101 1011 0000 0100   0x5B04
4:        0110 0001 0010 0100   0x6124
5:        0000 1111 0010 0000   0x0F20
6:        0010 0000 0001 1000   0x2018
7:        0001 0000 0001 1000   0x1018
8:        0000 1101 1110 0000   0x0DE0
9:        0000 0000 0000 0000   0x0000
10:       0000 0000 0000 0000   0x0000
11:       0000 0000 0000 0000   0x0000
12:       0000 0000 0000 0000   0x0000
13:       0000 0000 0000 0000   0x0000
14:       0000 0000 0000 0000   0x0000
15:       0000 0000 0000 0000   0x0000
16:       0000 0000 0000 0000   0x0000
17:       0000 0000 0000 0000   0x0000
18:       0000 0000 0000 0000   0x0000
19:       0000 0000 0000 0000   0x0000
20:       0000 0000 0000 0000   0x0000
21:       0000 0000 0000 0000   0x0000
22:       0000 0000 0000 0000   0x0000
23:       0000 0000 0000 0000   0x0000
24:       0000 0000 0000 0000   0x0000
25:       0000 0000 0000 0000   0x0000
*/