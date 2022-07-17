mod ram;
mod rom;

pub use ram::Ram;
pub use rom::Rom;

trait Memory {
    fn get_buffer(&self) -> &Vec<u8>;

    fn get_mut_buffer(&mut self) -> &mut Vec<u8>;

    fn read(&self, address: usize) -> u8 {
        self.get_buffer()[address]
    }

    fn write(&mut self, address: usize, value: u8) {
        self.get_mut_buffer()[address] = value
    }

    fn read_word(&self, address: usize) -> u16 {
        (self.read(address) as u16) << 8 | self.read(address + 1) as u16
    }

    fn write_word(&mut self, address: usize, value: u16) {
        self.write(address, (value >> 8) as u8);
        self.write(address + 1, value as u8);
    }
}
