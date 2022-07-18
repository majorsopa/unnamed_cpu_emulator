mod ram;

pub use ram::Ram;

trait Memory {
    fn get_buffer(&self) -> &Vec<u8>;

    fn get_mut_buffer(&mut self) -> &mut Vec<u8>;

    fn read_byte(&self, address: usize) -> u8 {
        self.get_buffer()[address]
    }

    fn write_byte(&mut self, address: usize, value: u8) {
        self.get_mut_buffer()[address] = value
    }

    fn read_word(&self, address: usize) -> u16 {
        (self.read_byte(address) as u16) << 8 | self.read_byte(address + 1) as u16
    }

    fn write_word(&mut self, address: usize, value: u16) {
        self.write_byte(address, (value >> 8) as u8);
        self.write_byte(address + 1, value as u8);
    }
}
