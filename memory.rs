//memory invariants
// Memory that is being allocated will never be allocated to a segment that is currently
// mapped
// Allocating and deallocating specific memory segments will not affect any other segments
// in memory
// The amount of words stored in a segment will never change from its initial length (unless
// unmapped then mapped with a different length)
// Each word will always be a u32.
// Attempting to map to already mapped segments will always fail
// Attempting to unmap already unmapped segments will always fail
// Attempting to access an unmapped segment that was previously mapped will act like a
// floating pointer
// Attempting to access a word out of the bounds of a segment’s length will always fail


pub struct MemObj {
    memory: Vec<Vec<u32>>,
    unmapped_memory: Vec<u32>,
}

impl MemObj{
    pub fn new(instructions: Vec<u32>) -> MemObj {
        let memory = vec![instructions];
        let unmapped_memory = vec![];
        MemObj { memory, unmapped_memory}
    }

    //loads program sequence of 32 bit words into mem[0]
    pub fn load_program(&mut self, dup_index: u32){
        self.memory[0] = self.memory[dup_index as usize].clone();
    }
        
    //allocates a new memory segment
    pub fn malloc(&mut self, seg_length: u32) -> u32{
        match self.unmapped_memory.pop() {
            Some(value) => {
                self.memory[value as usize] = vec![0; seg_length as usize];
                return value as u32;
            }
            None => {
                self.memory.push(vec![0; seg_length as usize]);
                return self.memory.len() as u32 -1;
            }
        }
    }
    
    //frees up a memory segment
    pub fn free(&mut self, mem_index: u32) {
        self.unmapped_memory.push(mem_index);
    }
    
    //returns reference to a specifc word in memort[seg_index][word_index]
    pub fn get_ref(&mut self, seg_index: u32, word_index: u32) -> &mut u32 {
        return &mut self.memory[seg_index as usize][word_index as usize];
    }
    
    //returns word in memort[seg_index][word_index]
    pub fn get_word(&self, seg_index: usize, word_index: usize) -> u32 {
        return self.memory[seg_index][word_index];
    }  
}