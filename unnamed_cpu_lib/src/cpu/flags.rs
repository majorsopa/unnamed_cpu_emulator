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
    
    pub fn set_carry(&mut self, value: bool) {
        if value {
            self.0 |= 1;
        } else {
            self.0 &= !1;
        }
    }
    
    pub fn set_zero(&mut self, value: bool) {
        if value {
            self.0 |= 2;
        } else {
            self.0 &= !2;
        }
    }
    
    pub fn set_negative(&mut self, value: bool) {
        if value {
            self.0 |= 4;
        } else {
            self.0 &= !4;
        }
    }
    
    pub fn get_carry(&self) -> bool {
        (self.0 & 1) != 0
    }
    
    pub fn get_zero(&self) -> bool {
        (self.0 & 2) != 0
    }
    
    pub fn get_negative(&self) -> bool {
        (self.0 & 4) != 0
    }
}