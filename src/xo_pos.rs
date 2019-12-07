pub trait XOPos {
    fn to_index(&self) -> u32;
}

pub struct Index(pub u32);
pub struct RowCol(pub u32, pub u32);
pub struct ColRow(pub u32, pub u32);

impl XOPos for u32 {
    fn to_index(&self) -> u32 {
        *self
    }
}

impl XOPos for Index {
    fn to_index(&self) -> u32 {
        self.0
    }
}

impl XOPos for RowCol {
    fn to_index(&self) -> u32 {
        (3 * self.0) + self.1
    }
}

impl XOPos for ColRow {
    fn to_index(&self) -> u32 {
        (3 * self.1) + self.0
    }
}
