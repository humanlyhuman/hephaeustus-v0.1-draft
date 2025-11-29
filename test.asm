    addi r1, r0, 5
    addi r2, r0, 1
loop:
    mul r2, r2, r1
    addi r1, r1, -1
    brz r1, done
    jmp r0, loop
done:
    addi r1, r2, 0
    addi r3, r0, 1
    syscall r3
    addi r3, r0, 0
    syscall r3