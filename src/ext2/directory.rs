use std::io::prelude::*;
use std::fs::File;

use crate::ext2::BLOCK_SIZE;

#[derive(Debug)]
struct DirectoryEntry {
    inode : u32,
    rec_len : u16,
    name_len : u8,
    file_type : u8,
    name : String
}

impl DirectoryEntry {
    pub fn write(&self, mut file : &File) -> std::io::Result<()> {
	println!("{:#?}", self);
	file.write(&self.inode.to_le_bytes())?;	
	file.write(&self.rec_len.to_le_bytes())?;	
	file.write(&self.name_len.to_le_bytes())?;	
	file.write(&self.file_type.to_le_bytes())?;	
	file.write(&self.name.as_bytes())?;
	file.seek(std::io::SeekFrom::Current((self.rec_len - 8 - self.name_len as u16) as i64))?;
	Ok(())
    }
}

#[derive(Debug)]
pub struct Directory {
    entries : Vec<DirectoryEntry>
}

impl Directory {
    pub fn new(inode : u32, parent_inode : u32) -> Self {
	let mut entries : Vec<DirectoryEntry> = Vec::new();
	let dot = DirectoryEntry {
	    inode: inode,
	    rec_len: 12,
	    name_len: 1,
	    file_type: 0,
	    name: String::from(".")
	};
	entries.push(dot);
	let dotdot = DirectoryEntry {
	    inode: parent_inode,
	    rec_len: BLOCK_SIZE as u16 - 12,
	    name_len: 2,
	    file_type: 0,
	    name: String::from("..")
	};
	entries.push(dotdot);
	Directory {
	    entries: entries
	}
    }

    pub fn write(&self, file : &File) -> std::io::Result<()> {
	for entry in &self.entries {
	    entry.write(&file)?;
	}
	Ok(())
    }

    pub fn len(&self) -> u32 {
	self.entries.iter().fold(0, |acc, x| acc + x.rec_len as u32)
    }
}
