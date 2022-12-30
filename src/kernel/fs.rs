use super::resource::Resource;
use alloc::{boxed::Box, string::String};
use bitflags::bitflags;

pub(crate) trait FileIO {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()>;
}

#[derive(Debug, Clone)]
pub(crate) struct File {
    contained_by: Directory,
    name: String,
    addr: u32,
    size: u32,
    offset: u32,
}

impl File {
    pub(crate) fn create(path: &str) -> Self {
        todo!();
    }

    pub(crate) fn open(path: &str) -> Option<Self> {
        todo!();
    }
}

impl FileIO for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        todo!();
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Directory {
    parent: Option<Box<Directory>>,
    name: String,
    addr: u32,
    size: u32,
}

impl Directory {
    pub(crate) fn open(path: &str) -> Option<Self> {
        todo!();
    }

    pub(crate) fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub(crate) fn entries(&self) -> DirEntries {
        DirEntries { dir: self.clone() }
    }
}

pub(crate) struct DirEntries {
    dir: Directory,
}

impl Iterator for DirEntries {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

impl FileIO for Directory {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        todo!();
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        Err(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FileKind {
    Dir = 0,
    File = 1,
}

#[derive(Clone)]
pub(crate) struct DirEntry {
    dir: Directory,
    name: String,
    addr: u32,
    kind: FileKind,
}

bitflags! {
    pub(crate) struct OpenFlag: u8 {
        const READ = 1;
        const WRITE = 1 << 1;
        const READWRITE = Self::READ.bits | Self::WRITE.bits;
        const APPEND = 1 << 2;
        const CREATE = 1 << 3;
        const TRUNCATE = 1 << 4;
        const DIR = 1 << 5;
        const DEVICE = 1 << 6;
    }

    pub(crate) struct SeekFlag: u8 {
        const START = 1;
        const CURRENT = 1 << 1;
        const END = 1 << 2;
    }
}

pub(crate) fn open(path: &str, flags: usize) -> Option<Resource> {
    let open_flag = OpenFlag::from_bits(flags as u8)?;

    if open_flag.contains(OpenFlag::DIR) {
        todo!();
    } else if open_flag.contains(OpenFlag::DEVICE) {
        todo!();
    } else {
        // we are opening a file

        if !(open_flag.contains(OpenFlag::READ) || open_flag.contains(OpenFlag::WRITE)) {
            return None;
        }

        let mut file = File::open(path);

        todo!();
    }
}
