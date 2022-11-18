// An interactive 

const a: usize = 0;
const b: usize = 1;
const c: usize = 2;
const s: usize = 3;


fn main() {
    let mut prg = start_program();
    println!("s is equal to {}", prg.registers[s]);
    prg.set(s, prg.val);
    println!("s is equal to {}", prg.registers[s]);
}

struct Program {
    registers: [usize; 4],
	r0: usize,
	r1: usize,
	val: u8,
    running: bool,
    cur_line: usize,
}

impl Program {
    fn adv(&mut self, r0: usize, val: u8) {
        self.registers[r0] += val as usize;
    }

    fn set(&mut self, r0: usize, val: u8) {
        self.registers[r0] = val as usize;
    }

    fn sbv(&mut self, r0: usize, val: u8) {
        self.registers[r0] -= val as usize;
    }

    fn add(&mut self, r0: usize, r1: usize) {
        self.registers[r0] += self.registers[r1];
    }

    fn sub(&mut self, r0: usize, r1: usize) {
        self.registers[r0] -= r1;
    }

    fn jmp(&mut self, val: u8) {
        self.cur_line = val as usize;
    }

    fn bez(&mut self, r0: usize, val: u8) {
        if self.registers[r0] == 0 {
            cur_line = (val + 1) as usize;
        }
    }

    fn spc(&mut self) {
        if self.registers[3] == 0 {
            // (do nothing)
        } else if self.registers[3] == 1 {
            // (get input)
        } else if self.registers[3] == 2 {
            println!("{}", self.registers[0])
        } else if self.registers[3] == 3 {
            self.running = false; 
        } else 
			self.running = false;
            println!("The program crashed due to an spc call with s value {}", self.registers[3]);
        }
    }
	
	fn end(&mut self) { 
	
		self.running = false;
		self.init_variables();
	}
	
	fn start_program() -> Program { 
		Program { 
			registers: [0; 4]
			r0: 0,
			r1: 0,
			val: 0,
			running: true,
			cur_line: 0,
		}
		
	
	fn init_variables(&mut self) {
	
		self.registers = [0; 4];
		self.val = 0;
		self.cur_line = 1;
	
	}
		
	
}
	

// copied from cbasm of course

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

// also copied straight from cbasm
fn swedish_to_decimal(string: String) -> i16 {
    let mut result = 0;

    // Hundreds
    if string.starts_with("tvåhundra") {
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

    if string.contains("åttio") {
        result += 70;
    }

    if string.contains("nittio") {
        result += 80;
    }
    // Big brain move ends

    // Ones and teens

    if string.ends_with("ett") {
        result += 1;
    } else if string.ends_with("två") {
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
    } else if string.ends_with("åtta") {
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