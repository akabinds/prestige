use super::{
    fs::{Directory, File, FileIO},
    io::console::Console,
};

#[derive(Debug, Clone)]
pub enum Resource {
    Device(Device),
    File(File),
    Directory(Directory),
}

impl FileIO for Resource {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        use Resource::*;

        match self {
            File(f) => f.read(buf),
            Device(dev) => dev.read(buf),
            Directory(dir) => dir.read(buf),
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        use Resource::*;

        match self {
            File(f) => f.write(buf),
            Device(dev) => dev.write(buf),
            Directory(dir) => dir.write(buf),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Device {
    Null,
    Console(Console),
}

impl FileIO for Device {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        use Device::*;

        match self {
            Null => Err(()),
            Console(c) => c.read(buf),
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        todo!();
    }
}
