use std::fs;
use std::io;

// This program should read through every line.
// If the line is valid code, convert it to an instruction (00..ff) and output it in a .cbx file
// If the line is empty or starts with '#', go to next line.
// If the line is invalid, do not assemble but instead return an error.

fn show_error(err: i16, line: u8) -> () {
    println!("{}", throw(err, line));
}

fn throw(err: i16, line: u8) -> String {
    return match err {
        0   => format!("Assembly successful"),
       -1   => format!("Failed to assemble with error code -1 (RegError): Invalid register at line {}", line + 1),
       -2   => format!("Failed to assemble with error code -2 (OpError): Invalid instruction at line {}", line + 1),
       -3   => format!("Failed to assemble with error code -3 (ValueError): Invalid value at line {}", line + 1),
       -4   => format!("Failed to assemble with error code -4 (SpaceError): Unexpected space at line {}", line + 1),
       -5   => format!("Failed to assemble with error code -5 (SelfJumpError): Self-referencing jump instruction at line {}", line + 1),
       -6   => format!("Failed to assemble with error code -6 (SegFaultError): Attempted invalid jump at line {}", line + 1),
       -128 => format!("Failed to assemble with error code -128 (OtherError): Unexpected error. Have fun debugging this one :)"),
       _ => panic!("Failed to assemble with UNEXPECTED error code ({}). This should never happen.", err),
   };
}

fn main() {
    let file: String = String::from("C:/cbcode/script.cb");
    let x: i16 = assemble(file);
    println!("Assembled the file! Returned {x}");
    //    println!("{}", throw(x));
    let output: String = String::new();
    //    let z = instruction_to_hex(String::from("15"));
    //    println!("{}", z)

    let mut line_output = line_to_instruction("adv a 0".to_string(), 0, 10);

    if line_output >= 0 {
        println!("{}", swedish_format(line_output));
    } else {
        println!("{}", throw(line_output, 0));
    }

    // Main loop
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        input = input.trim().to_string();
        println!("{:?}", input);

        line_output = line_to_instruction(input, 1, 10);

        if line_output >= 0 {
            println!(
                "\n{}\n{} (= {})\n",
                swedish_format(line_output),
                bin_format(line_output),
                swedish_to_decimal(swedish_format(line_output))
            );
        } else {
            println!("\n{}\n", throw(line_output, 0));
        }
    }
}

/// Attempts to assemble a .cb file, creating a .cbx file in the process.
/*
Returns              0   on successful assembly
                    -1   on RegError         (invalid register)
                    -2   on OpError          (invalid instruction)
                    -3   on ValueError         (invalid value)
                    -4   on SpaceError       (unexpected space; this mostly exists to help the user format correctly)
                    -5   on SelfJumpError    (attempting to jump to the same line)
                    -6   on SegFaultError    (attempting to jump to a line not in the program) [this is a horrible name for an error]
                    -128 on OtherError       (other error)
*/

fn assemble(file: String) -> i16 {
    let contents = fs::read_to_string(file).expect("");
    let lines: Vec<&str> = contents.lines().collect();
    let mut output: String = String::new();
    let mut current_line: u8 = 0;
    let max_lines = lines.len() as u8;

    // Now we actually assemble the damn thing....
    for line in &lines {
        let x = line_to_instruction(line.to_string(), current_line, max_lines);
        if x < 0 {
            show_error(x, current_line);
            return x;
        } else {
            output.push_str(&swedish_format(x));
            println!("{x}");
            output.push(' ');
        }
        current_line += 1;
    }
    println!("Output stored: {}", output);
    return 0;
}

/// Attempts to convert a line of .cb code to a Swedish number between "noll" and "tv??hundrafemtiofem" (however, in pra)
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
    if words.len() == 1 && words[0] == "" { // empty line
        return 0;
    } else if words[0].starts_with("#") { // comment
        return 0;
    }
    

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
        
        // set the jump target
        let mut jump_target: u8 = words[1].parse::<u8>().unwrap();
        if jump_target == 0 || jump_target > 32 {
            return -3;   // ValueError
        }
        jump_target -= 1;
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
                " " => return -4, // SpaceError
                _ => return -1,   // invalid register
            };

        // set the jump target
        let mut jump_target: u8 = words[2].parse::<u8>().unwrap();
        if jump_target == 0 {
            return -3; // ValueError
        }
        jump_target -= 1;
        if jump_target > 8 {
            return -3; // ValueError
        }
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

fn swedish_format(num: i16) -> String {
    if num == 0 {
        return String::from("noll");
    }

    let mut hundred_string = match (num / 100) {
        1 => "etthundra",
        2 => "tv??hundra",
        _ => "",
    }
    .to_string();
    let mut ten_string = String::new();
    let mut one_string = String::new();

    if num % 100 <= 10 || num % 100 >= 20 {
        ten_string = match (num - 100 * (num / 100)) / 10 {
            0 => "",
            1 => "tio",
            2 => "tjugo",
            3 => "trettio",
            4 => "fyrtio",
            5 => "femtio",
            6 => "sextio",
            7 => "sjuttio",
            8 => "??ttio",
            9 => "nittio",
            _ => panic!("Error when calculating ten_string for {}", num),
        }
        .to_string();

        one_string = match (num % 10) {
            0 => "",
            1 => "ett",
            2 => "tv??",
            3 => "tre",
            4 => "fyra",
            5 => "fem",
            6 => "sex",
            7 => "sju",
            8 => "??tta",
            9 => "nio",
            _ => panic!("Math error"),
        }
        .to_string();
    } else {
        ten_string = match (num % 100) {
            11 => "elva",
            12 => "tolv",
            13 => "tretton",
            14 => "fjorton",
            15 => "femton",
            16 => "sexton",
            17 => "sjutton",
            18 => "arton",
            19 => "nitton",
            _ => panic!("Error when calculating ten_string for teen number {}", num),
        }
        .to_string();
        one_string = String::new();
    }

    hundred_string.push_str(&ten_string);
    hundred_string.push_str(&one_string);

    return hundred_string;
}

fn swedish_to_decimal(string: String) -> i16 {
    let mut result = 0;

    // Hundreds
    if string.starts_with("tv??hundra") {
        result += 200;
    }

    if string.starts_with("etthundra") {
        result += 100;
    }

    // Tens

    if string.contains("tio") {
        result += 10;
    }

    if string.contains("tjugo") {
        result += 20;
    }

    // Big brain move
    if string.contains("trettio") {
        result += 20;
    }

    if string.contains("fyrtio") {
        result += 30;
    }

    if string.contains("femtio") {
        result += 40;
    }

    if string.contains("sextio") {
        result += 50;
    }

    if string.contains("sjuttio") {
        result += 60;
    }

    if string.contains("??ttio") {
        result += 70;
    }

    if string.contains("nittio") {
        result += 80;
    }
    // Big brain move ends

    // Ones and teens

    if string.ends_with("ett") {
        result += 1;
    } else if string.ends_with("tv??") {
        result += 2;
    } else if string.ends_with("tre") {
        result += 3;
    } else if string.ends_with("fyra") {
        result += 4;
    } else if string.ends_with("fem") {
        result += 5;
    } else if string.ends_with("sex") {
        result += 6;
    } else if string.ends_with("sju") {
        result += 7;
    } else if string.ends_with("??tta") {
        result += 8;
    } else if string.ends_with("nio") {
        result += 9;
    } else if string.ends_with("elva") {
        result += 11;
    } else if string.ends_with("tolv") {
        result += 12;
    } else if string.ends_with("tretton") {
        result += 13;
    } else if string.ends_with("fjorton") {
        result += 14;
    } else if string.ends_with("femton") {
        result += 15;
    } else if string.ends_with("sexton") {
        result += 16;
    } else if string.ends_with("sjutton") {
        result += 17;
    } else if string.ends_with("arton") {
        result += 18;
    } else if string.ends_with("nitton") {
        result += 19;
    }

    return result;
}

// format!("{:b}", line_to_instruction("set a 4".to_string(), 0, 1)));
