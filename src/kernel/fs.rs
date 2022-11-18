use alloc::{boxed::Box, string::String};

use super::resource::Resource;

pub trait FileIO {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()>;
}

#[derive(Debug, Clone)]
pub struct File {
    contained_by: Directory,
    name: String,
    addr: u32,
    size: u32,
    offset: u32,
}

impl File {
    pub fn create(path: &str) -> Self {
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
pub struct Directory {
    parent: Option<Box<Directory>>,
    name: String,
    addr: u32,
    size: u32,
}

impl Directory {
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn entries(&self) -> DirEntries {
        DirEntries { dir: self.clone() }
    }
}

pub struct DirEntries {
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
pub enum FileType {
    Dir = 0,
    File = 1,
}

#[derive(Clone)]
pub struct DirEntry {
    dir: Directory,
    name: String,
    addr: u32,
    kind: FileType,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum OpenFlag {
    Read = 1,
    Write = 2,
    Append = 3,
    Create = 4,
    Truncate = 5,
    ReadWrite = 6,
    Dir = 7,
    Device = 8,
}

impl OpenFlag {
    fn is_set(&self, flags: usize) -> bool {
        flags & (*self as usize) != 0
    }
}

pub fn open(path: &str, flags: usize) -> Option<Resource> {
    if OpenFlag::Dir.is_set(flags) {
        todo!();
    } else if OpenFlag::Device.is_set(flags) {
        todo!();
    } else {
        if !(OpenFlag::Read.is_set(flags)
            || OpenFlag::Write.is_set(flags)
            || OpenFlag::ReadWrite.is_set(flags))
        {
            None
        } else {
            todo!();
        }
    }
}
