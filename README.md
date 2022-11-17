# mouanga-assembly
C♭ (pronounced c-flat, or "cess" in Swedish) is like C but one step lower*, just like you would expect. It also goes by the name mouanga-assembly due to the simple reason that the developer of this language was mystically compelled (forced) to name it that way.

\* Lower in terms of creativity, usability, readability, well in every single positive metric actually.

**Note: When writing C♭ code, it's best practice to only use lowercase letters.**


## Registers



**A**, **B** and **C** are normal registers. **S** is read by the **SPC** instruction, which can then modify the value of **A**.

## Instruction set


ADV $r0, %val       (0b000 | 0) - set $r0 to $r0 + %val. Note: registers can overflow.

SET $r0, %val       (0b001 | 1) - set $r0 to %val

SBV $r0, %val        (0b010 | 2) - set $r0 to $r0 - val.

ADD $r0, $r1        (0b011 | 3) - set $r0 to $r0 + $r1. Note: registers can overflow

SUB $r0, $r1        (0b100 | 4) - set $r0 to $r0 - $r1. Note: registers can underflow

JMP %val             (0b101 | 5) - jump to line number %val

BEZ $r0, $val        (0b110 | 6) - if $r0 is 0, jump to line %val, else continue program

SPC                 (0b111 | 7) - performs an action depending on the value of **S**


### SPC actions


0x00000000  | 0  - do nothing

0x00000001  | 1  - get input through standard input, and store it in **S**

0x00000002  | 2  - print **A** as integer

0x00000003  | 3  - end program

0x00000004+ | 4+ - crash
