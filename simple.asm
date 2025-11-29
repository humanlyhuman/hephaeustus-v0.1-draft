; Simple addition test (4-bit immediates: -8 to 7)
addi r1,0 r, 5
addi r2, r0, 7
add r3, r1, r2
addi r1, r3, 0
addi r3, r0, 1
syscall r3
addi r3, r0, 0
syscall r3
