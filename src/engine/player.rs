use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum Player {
    White = 8,
    Black = 16,
}
