// basically just a bitfield
/* 
order of flags:
0: carry flag
1: zero flag
2: negative flag
*/
pub struct Flags(u16);

impl Flags {
    pub fn new() -> Flags {
        Flags(0)
    }

    pub fn set_flag(&mut self, flag: u16) {
        self.0 |= flag;
    }

    pub fn clear_flag(&mut self, flag: u16) {
        self.0 &= !flag;
    }

    pub fn get_flag(&self, flag: u16) -> bool {
        (self.0 & flag) == flag
    }

    pub fn set_flag_to_bool(&mut self, flag: u16, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }
}

pub enum FlagAliases {
    Carry,
    Zero,
    Negative,
}