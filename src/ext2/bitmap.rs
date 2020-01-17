use std::io::prelude::*;
use std::fs::File;
use std::vec::Vec;
use std::convert::TryInto;

use crate::ext2::BLOCK_SIZE;

use crate::ext2::superblock::Superblock;
use crate::ext2::bgd::BGD;

#[derive(Debug)]
pub struct Bitmap {
    values: Vec<u8>
}

impl Bitmap {
    pub fn set(&mut self, idx: u32, val: bool) {
	if val {
	    self.values[idx as usize / 8] |= 1 << (idx % 8);
	} else {
	    self.values[idx as usize / 8] &= !(1 << (idx % 8));
	}
    }

    pub fn get(&self, idx: u32) -> bool {
	(self.values[idx as usize / 8] & (1 << (idx % 8))) != 0
    }

    pub fn len(&self) -> u32 {
	(self.values.len() * 8).try_into().unwrap()
    }

    pub fn new(len: u32) -> Self {
	Self {
	    values: vec![0; (len as usize + 7) / 8]
	}
    }

    pub fn alloc(&mut self, bgd : &mut BGD, sb : &mut Superblock) -> Option<u32> {
	for i in 0 .. self.len() {
	    if ! self.get(i) {
		bgd.bg_free_blocks_count -= 1;
		sb.s_free_blocks_count -= 1;
		self.set(i, true);
		if bgd.idx == 0 {
		    return Some(i + 1);
		}
		return Some(i);
	    }
	}
	None
    }

    pub fn write(&self, mut file : &File) -> std::io::Result<()> {
	file.write(&self.values)?;
	for _ in 0 .. BLOCK_SIZE as usize - self.values.len() {
	    file.write(&[0xff])?;
	}
	Ok(())
    }
}
