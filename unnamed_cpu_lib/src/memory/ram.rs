use super::Memory;

pub struct Ram(Vec<u8>);

impl Ram {
    pub fn new(size: u16) -> Self {
        Self(vec![0; size.into()])
    }
}

// todo make this a derive macro
impl Memory for Ram {
    fn get_buffer(&self) -> &Vec<u8> {
        &self.0
    }

    fn get_mut_buffer(&mut self) -> &mut Vec<u8> {
        &mut self.0
    }
}
