use std::fs::File;
use std::io::prelude::*;

// This program should read through every line.
// If the line is valid code, convert it to an instruction (00..ff) and output it in a .cbx file
// If the line is empty or starts with '#', go to next line.
// If the line is invalid, do not assemble but instead return an error.

fn main() {
    let file: String = String::new();
    let x: u8 = assemble(file);
    match x {
        0 => println!("Assembly successful"),
        1 => println!("Failed to assemble with error code 1 (RegError): Invalid registry at X"),
        2 => println!("Failed to assemble with error code 2 (OpError): Invalid instruction at X"),
        3 => println!("Failed to assemble with error code 3 (NumError): Invalid 32-bit integer value at X"),
        4 => println!("Failed to assemble with error code 4 (SpaceError): Unexpected space at X"),
        5 => println!("Failed to assemble with error code 5 (SelfJumpError): Self-referencing jump instruction at X"),
        6 => println!("Failed to assemble with error code 6 (SegFaultError): Attmpted to jump to instruction X, but there are only Y instructions in the program"),
        255 => println!("Failed to assemble with error code 255 (OtherError): Unexpected error. Have fun debugging this one :)"),
        _ => panic!("Failed to assemble with UNEXPECTED error code ({}). This should never happen.", x),
    };
    let input: String = String::new();
    let output: String = String::new();
    let z = instruction_to_hex(String::from("15"));
    println!("{}", z)
}

/// Attempts to assemble a .cb file, creating a .cbx file in the process.
/*
Returns             0   on successful assembly
                    1   on RegError         (invalid registry)
                    2   on OpError          (invalid instruction)
                    3   on NumError         (invalid value)
                    4   on SpaceError       (unexpected space; this mostly exists to help the user format correctly)
                    5   on SelfJumpError    (attempting to jump to the same line)
                    6   on SegFaultError    (attempting to jump to a line not in the program) [this is a horrible name for an error]
                    255 on OtherError       (other error)
*/

fn assemble(file: String) -> u8 {
    return 0;
    // todo
}

/// Attempts to convert a line of .cb code to a hex between 0..ff.
fn convert_line(line: String) -> String {
    return String::new();
}

/// Attempts to convert an instruction (0..255) to a hex between 0..ff.
fn instruction_to_hex(inst: String) -> String {
    let instruction_number: u8 = inst.parse().unwrap();
    return format!("{:x}", instruction_number);
}

/// Attempts to convert a line of .cb code to an instruction between 0..255.
fn line_to_instruction(line: String) -> Result <u8, String> {
    let instruction: u8 = 0;
    let words: Vec<&str> = line.split(' ').collect();
    
    // set the opcode to the leftmost bits of the instruction
    instruction += 0b00010000 * match words[0] {
        "adv" => 0b000,
        "set" => 0b001,
        "cpy" => 0b010,
        "add" => 0b011,
        "sub" => 0b100,
        "jmp" => 0b101,
        "bez" => 0b110,
        "spc" => 0b111,
    }

    // then depending on said opcode, change remaining bits accordingly

    // adv and set are both of the type "$r0, %val" where $r0 is a 2-bit register and %val is a 3-bit immediate value
    if (match instruction/0b00010000{
        0b000 => true,
        0b001 => true,
        _     => false,
    }) {
        instruction += 0b
    }



}
