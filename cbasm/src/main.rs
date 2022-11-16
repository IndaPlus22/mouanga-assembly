// use File::*;

// This program should read through every line.
// If the line is valid code, convert it to an instruction (00..ff) and output it in a .cbx file
// If the line is empty or starts with '#', go to next line.
// If the line is invalid, do not assemble but instead return an error.



fn main() {
    let file: String = String::new();
    let result = match assemble(file) {
        0 => println!("Assembly successful."),
        1 => println!("Failed to assemble with error code 1 (RegError): Invalid registry at X"),
        2 => println!("Failed to assemble with error code 2 (OpError): Invalid instruction at X"),
        3 => println!("Failed to assemble with error code 3 (NumError): Invalid 32-bit integer value at X"),
        4 => println!("Failed to assemble with error code 4 (SpaceError): Unexpected space at X"),
        5 => println!("Failed to assemble with error code 5 (SelfJumpError): Self-referencing jump command at X"),
        6 => println!("Failed to assemble with error code 6 (SegFaultError): Attmpted to jump to instruction X, but there are only Y instructions in the program"),
        _ => println!("Failed to assemble with error code 255 (OtherError): Unexpected error. Have fun debugging this one :)"),
    };
    let input:  String = String::new();
    let output: String = String::new();
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
    return 55;
    // todo
}

fn convert_line(line: String) -> String { 
    // todo
    return String::new();
}