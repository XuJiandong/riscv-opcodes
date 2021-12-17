use log::debug;
use std::fmt;
use std::io::{self, BufRead};

struct Field {
    pub start: usize,
    pub length: usize,
    pub value: Option<u32>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "start = {}, length = {}, value = {:?}",
            self.start, self.length, self.value
        )
    }
}

const INVALID_START: usize = 0xFFFF;

fn parse_arg(arg: &str) -> Field {
    let length = match arg {
        "vd" => 5,
        "vs1" => 5,
        "vs2" => 5,
        "vs3" => 5,
        "vm" => 1,
        "nf" => 3,
        "wd" => 1,
        "simm5" => 5,
        "zimm" => 5,
        "zimm10" => 10,
        "zimm11" => 11,
        "rd" => 5,
        "rs1" => 5,
        "rs2" => 5,
        _ => panic!("Unknown arg {}", arg),
    };
    // `start` should be updated later
    Field {
        start: INVALID_START,
        length,
        value: None,
    }
}

// hi..lo=value or bit=value (e.g. 6..2=0x45 10=1)
fn parse_pair(pair: &str) -> Field {
    let kv: Vec<&str> = pair.split("=").map(|s| s.trim()).collect();
    let key = kv[0];
    let value = kv[1];
    let value = u32::from_str_radix(&value.replace("0x", ""), 16).unwrap();
    let keys: Vec<&str> = key.split("..").map(|s| s.trim()).collect();
    if keys.len() == 1 {
        // bit=value
        let start = keys[0].parse::<usize>().unwrap();
        Field {
            start,
            length: 1,
            value: Some(value),
        }
    } else if keys.len() == 2 {
        // hi..lo=value
        let hi = keys[0].parse::<usize>().unwrap();
        let lo = keys[1].parse::<usize>().unwrap();
        Field {
            start: lo,
            length: hi - lo + 1,
            value: Some(value),
        }
    } else {
        panic!("Invalid pair {}", pair);
    }
}

// generating rust code like below:
// x if x & 0b_111_111_1_11111_00000_111_00000_1111111 == 0b_000_000_1_01011_00000_000_00000_0000111 => Some(insts::OP_VLM_V),
// https://github.com/nervosnetwork/ckb-vm/blob/2d096ed038694b86b5e0ca15c75c7758bb655a78/src/instructions/v.rs#L468
fn gen_rust(name: &str, fields: &Vec<Field>) -> String {
    let mut mask = String::from("0b");
    let mut dest = String::from("0b");
    for field in fields {
        mask += "_";
        dest += "_";
        let digit = if let Some(value) = field.value {
            let bin = format!("{:032b}", value);
            let bin = bin.as_str();
            dest += &bin[bin.len() - field.length..];
            "1"
        } else {
            dest += "0".repeat(field.length).as_ref();
            "0"
        };
        mask += digit.repeat(field.length).as_ref();
    }
    let new_name = String::from("OP_") + &name.replace(".", "_").to_uppercase();
    format!("x if x & {} == {} => Some(insts::{})", mask, dest, new_name)
}

fn main() {
    env_logger::init();
    let stdin = io::stdin();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let line = line.trim();
        if line.len() == 0 || (line.len() > 0 && line.chars().nth(0).unwrap() == '#') {
            // comment or empty line
            continue;
        }
        debug!("process line: {}", line);
        let split = line.split(char::is_whitespace);
        let field_strs: Vec<&str> = split.map(|s| s.trim()).filter(|s| s.len() > 0).collect();
        let mut fields = Vec::<Field>::new();
        for str in &field_strs[1..] {
            let field = if str.contains("=") {
                parse_pair(str)
            } else {
                parse_arg(str)
            };
            fields.push(field);
        }
        let mut accumulated_start = 0;
        let mut total_length = 0;
        for index in (0..fields.len()).rev() {
            if fields[index].start == INVALID_START {
                fields[index].start = accumulated_start;
            }
            accumulated_start += fields[index].length;
            total_length += fields[index].length;
        }
        if total_length != 32 {
            panic!("process\n{}\nerror, total length is {}", line, total_length);
        }

        for f in &fields {
            debug!("field: {}", f);
        }

        let code = gen_rust(field_strs[0], &fields);
        println!("{}", code);
    }
}
