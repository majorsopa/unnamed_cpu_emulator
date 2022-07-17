use super::Memory;

pub struct Rom(Vec<u8>);

impl Rom {
    pub fn new(size: usize) -> Self {
        Self(vec![0; size])
    }
}

// todo make this a derive macro
impl Memory for Rom {
    fn get_buffer(&self) -> &Vec<u8> {
        &self.0
    }

    fn get_mut_buffer(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}
