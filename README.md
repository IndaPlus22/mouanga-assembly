# mouanga-assembly
C♭ (pronounced c-flat, or "cess" in Swedish) is like C but one step lower*, just like you would expect. It also goes by the name mouanga-assembly due to the simple reason that the developer of this language was mystically compelled (forced) to name it that way.

\* Lower in terms of creativity, usability, readability, well in every single positive metric actually.


## Instruction set


NUL                 (0b000) - do nothing

ADD $r0, $r1, $r2   (0b001) - calculate $r1 + $r2 and store it in $r0. Note: all registries are unsigned and can overflow

SUB $r0, $r1, $r2   (0b010) - calculate $r1 - $r2 and store it in $r0. Note: all registries are unsigned and can underflow

JMP $ln             (0b010) - jump to line number $ln

BEQ $r0, $r1, $ln   (0b011) - if $r0 and $r1 are equal, jump to line $ln, else continue program
