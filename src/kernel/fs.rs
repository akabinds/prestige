pub trait FileIO {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()>;
}

#[repr(u8)]
pub enum OpenFlag {
    Read = 1,
    Write = 2,
    Append = 3,
    Create = 4,
    Truncate = 5,
}
