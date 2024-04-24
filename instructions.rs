use std::io::Read;
use std::io::Write;
use std::io::{stdin, stdout};
use crate::memory::MemObj;

pub struct Register {
    registers:[u32; 8],
}

impl Register {
    pub fn new(registers:[u32; 8]) -> Register{
        Register{registers}
    }

    //Function would set value of registers[ra] to registers[rb] if registers[rc] is not 0
    pub fn conditional_move(&mut self, ra: u8, rb:u8,  rc: u8) {
        if self.registers[rc as usize] != 0 {
            self.registers[ra as usize] = self.registers[rb as usize];
        }
    }

    // Function sets registers[ra] to the word in memory located at segment registers[rb] word registers[rc]
    pub fn seg_load(&mut self, ra: u8, rb:u8,  rc: u8, memory: &MemObj ) {
        self.registers[ra as usize] = memory.get_word( self.registers[rb as usize] as usize, self.registers[rc as usize] as usize);
    }

    // Lets word in memory located at segment registers[ra] word registers[rb] to the value of registers[rc]
    pub fn seg_store(&self, ra: u8, rb:u8,  rc: u8, memory: &mut MemObj){
        *memory.get_ref(self.registers[ra as usize], self.registers[rb as usize]) = self.registers[rc as usize];
    }

    // Sets registers[ra] = (registers[rb] + registers[rc]) mod 2^32
    pub fn add(&mut self, ra: u8, rb:u8,  rc: u8){
        self.registers[ra as usize] = self.registers[rb as usize].wrapping_add(self.registers[rc as usize] ) ;
    }

    // Sets registers[ra] = (registers[rb] * registers[rc]) mod 2^32
    pub fn mult(&mut self, ra: u8, rb:u8,  rc: u8){
        self.registers[ra as usize] = self.registers[rb as usize].wrapping_mul(self.registers[rc as usize]);
    }

    // Sets registers[ra] = (registers[rb] / registers[rc])
    pub fn div(&mut self, ra: u8, rb:u8,  rc: u8){
        self.registers[ra as usize] = self.registers[rb as usize] / self.registers[rc as usize];
    }

    // Sets registers[ra] to registers[rb] nanded with registers[rc]
    pub fn nand(&mut self, ra: u8, rb:u8,  rc: u8){
        self.registers[ra as usize] = !(self.registers[rb as usize] & self.registers[rc as usize]);
    }

    //halts execution
    pub fn halt(){
        std::process::exit(0);
    }

    // Make a new memory segment of registers[rc] words (that are all 0) that is stored in registers[rb], then maps it to memory using functions defined in the memory.rs module
    pub fn map_segment(&mut self, rb:u8,  rc: u8, memory: &mut MemObj) {
        self.registers[rb as usize] = memory.malloc( self.registers[rc as usize]);
    }

    // Uses the free function in memory.rs to unmap the segment in memory registers[rc]
    pub fn unmap_segment(&self, rc: u8, memory: &mut MemObj) {
        memory.free(self.registers[rc as usize]);
    }

    // Outputs registers[rc] to standard out
    pub fn output(&self, rc: u8) {
        stdout().write_all(&[self.registers[rc as usize] as u8]).unwrap();
        stdout().flush().unwrap();
    }

    // Gets a value from standard input and stores it in registers[rc]
    pub fn input(&mut self, rc: u8) {
        self.registers[rc as usize] = stdin().bytes().next().unwrap().unwrap() as u32;
    }

    //loads program into segment 0 and unpdates program counter
    pub fn load_program(&self, rb:u8, rc:u8, memory: &mut MemObj) ->usize {
        if self.registers[rb as usize] != 0 {
            memory.load_program(self.registers[rb as usize]);
        }
        return self.registers[rc as usize] as usize;
    }

    //sets reg[ra] to value
    pub fn load_value(&mut self, ra:u8, value:u32){
        self.registers[ra as usize] = value;
    }
}