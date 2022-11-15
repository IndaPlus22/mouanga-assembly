# mouanga-assembly
C♭ (pronounced c-flat, or "cess" in Swedish) is like C but one step lower*, just like you would expect. It also goes by the name mouanga-assembly due to the simple reason that the developer of this language was mystically compelled (forced) to name it that way.

\* Lower in terms of creativity, usability, readability, well in every single positive metric actually.


## Instruction set


NUL                 (0b000 | 0) - do nothing

SET $r0, %val       (0b001 | 1) - set $r0 to %val

CPY $r0, $r1        (0b010 | 2) - set $r0 to $r1

ADD $r0, $r1, $r2   (0b011 | 3) - calculate $r1 + $r2 and store it in $r0. Note: registers can overflow

SUB $r0, $r1, $r2   (0b100 | 4) - calculate $r1 - $r2 and store it in $r0. Note: registers can underflow

JMP %ln             (0b101 | 5) - jump to line number %ln

BEQ $r0, $r1, $ln   (0b110 | 6) - if $r0 and $r1 are equal, jump to line $ln, else continue program

SPC                 (0b111 | 7) - performs an action depending on the value of **S**


### SPC actions

0x00000000  | 0  - do nothing

0x00000001  | 1  - get input through standard input, and store it in **A**

0x00000002  | 2  - print **A** as integer

0x00000003+ | 3+ - crash
