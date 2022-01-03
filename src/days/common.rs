pub trait BitArr {
    fn get(&self, index: Self) -> bool;
    fn set(&mut self, index: Self);
    fn clear(&mut self, index: Self);
}

impl BitArr for usize {
    fn get(&self, index: Self) -> bool {
        (self >> index) & 1 != 0
    }
    fn set(&mut self, index: Self) {
        *self |= 1 << index;
    }
    fn clear(&mut self, index: Self) {
        *self &= !(1 << index);
    }
}
