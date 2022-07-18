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
        (self.read_byte(address) as u16) << 8 | self.read_byte(address + 1) as u16
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value >> 8) as u8);
        self.write_byte(address + 1, value as u8);
    }
}
