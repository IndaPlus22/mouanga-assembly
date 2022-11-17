use std::fs::File;
use std::io::prelude::*;

// This program should read through every line.
// If the line is valid code, convert it to an instruction (00..ff) and output it in a .cbx file
// If the line is empty or starts with '#', go to next line.
// If the line is invalid, do not assemble but instead return an error.

fn throw(err: i16) -> String {
    return match err {
        0   => String::from("Assembly successful"),
       -1   => String::from("Failed to assemble with error code -1 (RegError): Invalid register at X"),
       -2   => String::from("Failed to assemble with error code -2 (OpError): Invalid instruction at X"),
       -3   => String::from("Failed to assemble with error code -3 (NumError): Invalid value at X"),
       -4   => String::from("Failed to assemble with error code -4 (SpaceError): Unexpected space at X"),
       -5   => String::from("Failed to assemble with error code -5 (SelfJumpError): Self-referencing jump instruction at X"),
       -6   => String::from("Failed to assemble with error code -6 (SegFaultError): Attmpted to jump to instruction X, but there are only Y instructions in the program"),
       -128 => String::from("Failed to assemble with error code -128 (OtherError): Unexpected error. Have fun debugging this one :)"),
       _ => panic!("Failed to assemble with UNEXPECTED error code ({}). This should never happen.", err),
   };
}

fn main() {
    let file: String = String::new();
    let x: i16 = assemble(file);
//    println!("{}", throw(x));
    let input: String = String::new();
    let output: String = String::new();
//    let z = instruction_to_hex(String::from("15"));
//    println!("{}", z)

let line_output = line_to_instruction("adv a 0".to_string(), 0, 10);

if line_output >= 0 {
println!("{}", 
bin_format(line_output
));
} else {
    println!("{}", throw(line_output));
}
}

/// Attempts to assemble a .cb file, creating a .cbx file in the process.
/*
Returns              0   on successful assembly
                    -1   on RegError         (invalid register)
                    -2   on OpError          (invalid instruction)
                    -3   on NumError         (invalid value)
                    -4   on SpaceError       (unexpected space; this mostly exists to help the user format correctly)
                    -5   on SelfJumpError    (attempting to jump to the same line)
                    -6   on SegFaultError    (attempting to jump to a line not in the program) [this is a horrible name for an error]
                    -128 on OtherError       (other error)
*/

fn assemble(file: String) -> i16 {
    return -5;
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
fn line_to_instruction(line: String, cur_line: u8, max_lines: u8) -> i16 {
    // Success values are >= 0, fail values are < 0
    let mut instruction: u8 = 0;
    let words: Vec<&str> = line.split(' ').collect();

    // set the opcode to the leftmost bits of the instruction
    instruction += 0b00100000
        * match words[0] {
            "adv" => 0b000,
            "set" => 0b001,
            "sbv" => 0b010,
            "add" => 0b011,
            "sub" => 0b100,
            "jmp" => 0b101,
            "bez" => 0b110,
            "spc" => 0b111,
            " " => return -4, // spaceerror
            _ => return -2,   // invalid instruction
        };

    // then depending on said opcode, change remaining bits accordingly

    // adv, set and sbv are of the type "$r0, %val" where $r0 is a 2-bit register and %val is a 3-bit immediate value
    if (match instruction / 0b00100000 {
        0b000 => true, // adv
        0b001 => true, // set
        0b010 => true, // sbv
        _ => false,
    }) {
        // set the register
        instruction += 0b00001000
            * match words[1] {
                "a" => 0b00,
                "b" => 0b01,
                "c" => 0b10,
                "s" => 0b11,
                " " => return -4, // spaceerror
                _ => return -1,   // invalid register
            };

        // set the immediate value
        if (words[2].parse::<u8>().unwrap() <= 0b111) {
            instruction += words[2].parse::<u8>().unwrap();
        } else {
            return -3; // invalid value
        }

    // add and sub are of the type "$r0, $r1" where $r0 and $r1 are 2-bit registers
    } else if (match instruction / 0b00100000 {
        0b011 => true, // add
        0b100 => true, // sub
        _ => false,
    }) {
        // set the register
        instruction += 0b00001000
            * match words[1] {
                "a" => 0b00,
                "b" => 0b01,
                "c" => 0b10,
                "s" => 0b11,
                " " => return -4, // spaceerror
                _ => return -1,   // invalid register
            };

        // set the other register
        instruction += 0b00000010
            * match words[2] {
                "a" => 0b00,
                "b" => 0b01,
                "c" => 0b10,
                "s" => 0b11,
                " " => return -4, // spaceerror
                _ => return -1,   // invalid register
            };
    // jmp is of the type "%val"
    } else if (instruction / 0b00100000 == 0b101) {
        // jump

        let jump_target: u8 = words[1].parse().unwrap();

        if jump_target > max_lines {
            return -6; // SegFaultError
        } else if jump_target == cur_line {
            return -5; // SelfJumpError
        } else {
            instruction += jump_target;
        }

    // bez is of the type "$r0, %val"
    } else if (instruction / 0b00100000 == 0b110) {
        // set the register
        instruction += 0b00001000
            * match words[1] {
                "a" => 0b00,
                "b" => 0b01,
                "c" => 0b10,
                "s" => 0b11,
                " " => return -4, // spaceerror
                _ => return -1,   // invalid register
            };

        // set the jump target
        let jump_target: u8 = words[1].parse().unwrap();

        if jump_target > max_lines {
            return -6; // SegFaultError
        } else if jump_target == cur_line {
            return -5; // SelfJumpError
        } else {
            instruction += jump_target;
        }

        // spc is of the type ""
    } else if (instruction / 0b00100000 == 0b111) {
    } else {
        panic!("")
    }

    return instruction.into();
}

fn bin_format(num: i16) -> String {
    let mut start_string = "";
    if num >= 0b0 {
        if num < 0b10 {
            start_string = "0000000";
        } else if num < 0b100 {
            start_string = "000000";
        } else if num < 0b1000 {
            start_string = "00000"
        } else if num < 0b10000 {
            start_string = "0000";
        } else if num < 0b100000 {
            start_string = "000";
        } else if num < 0b1000000 {
            start_string = "00";
        } else if num < 0b10000000 {
            start_string = "0";
        }
    }
    return format!("{}{:b}", start_string, num);
}

// format!("{:b}", line_to_instruction("set a 4".to_string(), 0, 1)));