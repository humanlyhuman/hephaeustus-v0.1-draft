start:
    addi r1, r0, 42
    addi r0, r0, 1
    syscall r0
    addi r0, r0, 0
    addi r1, r0, 0
    syscall r0
