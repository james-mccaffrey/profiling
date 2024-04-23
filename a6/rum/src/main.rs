use std::env;
use bitpack;
use rum::instructions::Register;
use rum::memory::MemObj;
use std::convert::TryInto;
use std::time::Instant;

//got this from the rumdump lab
pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        
        Some(filename) => {
            match std::fs::File::open(filename) {
                Err(_) => std::process::exit(1),
                Ok(val) =>Box::new(std::io::BufReader::new(val,)),
            }
        }
    };
    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();
    let instructions: Vec<u32> = buf.chunks_exact(4).map(|x| u32::from_be_bytes(x.try_into().unwrap())).collect();
    instructions
}

fn main() {
    let input = env::args().nth(1);
    let mut regs = Register::new([0,0,0,0,0,0,0,0]);
    let mut mem = MemObj::new(load(input.as_deref()));
    let mut program_counter: usize = 0;
    let now = Instant::now();
    loop{
        let instruction = mem.get_word(0, program_counter);
        program_counter+=1;

        let ra = bitpack::bitpack::getu(instruction as u64, 3, 6).unwrap() as u8;
        let rb = bitpack::bitpack::getu(instruction as u64, 3, 3).unwrap() as u8;
        let rc = bitpack::bitpack::getu(instruction as u64, 3, 0).unwrap() as u8;
        let op = bitpack::bitpack::getu(instruction as u64, 4, 28).unwrap() as u8;

        match op {
            0 => {
                Register::conditional_move(&mut regs, ra, rb, rc);
            },
            1 => {
                Register::seg_load(&mut regs, ra, rb, rc, &mut mem);
            },
            2 => {
                Register::seg_store(&regs, ra, rb,  rc, &mut mem);
            },
            3 => {
                Register::add(&mut regs, ra, rb,  rc);
            }
            4 => {
                Register::mult(&mut regs, ra, rb,  rc);
            },
            5 => {
                Register::div(&mut regs, ra, rb,  rc);
            },
            6 => {
                Register::nand(&mut regs, ra, rb,  rc);
            },
            7 => {
                let elapsed = now.elapsed();
                eprintln!("{:.2?}", elapsed);
                Register::halt();
            },
            8 => {
                Register::map_segment(&mut regs, rb, rc, &mut mem);
            },
            9 => {
                Register::unmap_segment(&regs, rc, &mut mem);
            },
            10 => {
                Register::output(&regs, rc);
            },
            11 => {
                Register::input(&mut regs, rc);
            },
            12 => {
                program_counter = Register::load_program(&regs, rb, rc, &mut mem);
            },
            13 => {
                let rl = bitpack::bitpack::getu(instruction as u64, 3, 25).unwrap() as u8;
                let vl = bitpack::bitpack::getu(instruction as u64, 25, 0).unwrap() as u32;
                Register::load_value(&mut regs, rl, vl);
            },
            _ => {
                println!("INVALID INSTRUCTION");
            }
        }
    }
}