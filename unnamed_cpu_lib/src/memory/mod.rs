mod ram;

pub use ram::Ram;

pub trait Memory {
    fn get_buffer(&self) -> &Vec<u8>;

    fn get_mut_buffer(&mut self) -> &mut Vec<u8>;

    fn read_byte(&self, address: u16) -> u8 {
        self.get_buffer()[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.get_mut_buffer()[address as usize] = value
    }

    fn read_word(&self, address: u16) -> u16 {
        // reads 2 bytes from the memory and returns them concatenated into a u16
        (self.read_byte(address) as u16) | (self.read_byte(address + 1) as u16)
    }

    fn write_word(&mut self, address: u16, value: u16) {
        // writes the 2 bytes to the memory in seperate 2 `write_byte`s
        self.write_byte(address, value as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }
}
