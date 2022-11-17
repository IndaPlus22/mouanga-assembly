// An interactive 

const EXIT_STRINGS: [&str; 2] = [
    "SUCCESS",  // 0
    "SPC_CRASH" // 1
    ];

fn main() {
    let mut prg = Program {
        a: 0,
        b: 0,
        c: 0,
        s: 0,
        running: true,
        cur_line: 1,
    };
    println!("s is equal to {}", prg.s);
    prg.set(self.s, 3);
    println!("s is equal to {}", prg.s);
}

struct Program {
    a: i32,
    b: i32,
    c: i32,
    s: i32,
    running: bool,
    cur_line: i32,
}

impl Program {
    fn adv(&mut self, r0: &mut i32, val: u8) {
        *r0 += val as i32;
    }

    fn set(&mut self, r0: &mut i32, val: u8) {
        *r0 = val as i32;
    }

    fn sbv(&mut self, r0: &mut i32, val: u8) {
        *r0 -= val as i32;
    }

    fn add(&mut self, r0: &mut i32, r1: i32) {
        *r0 += r1;
    }

    fn sub(&mut self, r0: &mut i32, r1: i32) {
        *r0 -= r1;
    }

    fn jmp(&mut self, val: u8, cur_line: &mut i32) {
        *cur_line = val as i32;
    }

    fn bez(&mut self, r0: i32, val: u8, cur_line: &mut i32) {
        if r0 == 0 {
            *cur_line = (val + 1) as i32;
        }
    }

    fn spc(&mut self) {
        if self.s == 0 {
            // (do nothing)
        } else if self.s == 1 {
            // (get input)
        } else if self.s == 2 {
            println!("{}", self.a)
        } else if self.s == 3 {
            self.running = false; 
        } else {
            println!("The program crashed due to an spc call with s value {}", self.s);
        }
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