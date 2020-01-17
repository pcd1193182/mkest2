use std::io::prelude::*;
use std::fs::File;
use std::convert::TryInto;

use crate::ext2::superblock::Superblock;

#[derive(Default)]
#[derive(Debug)]
pub struct BGD {
    pub bg_block_bitmap : u32,
    pub bg_inode_bitmap : u32,
    pub bg_inode_table : u32,
    pub bg_free_blocks_count : u16,
    pub bg_free_inodes_count : u16,
    pub bg_used_dirs_count : u16,
    pub idx : u32
}
const BGD_PADDING : i64 = 14;
pub const BGD_SIZE : u32 = 32;

impl BGD {
    pub fn write(&self, mut file : &File) -> std::io::Result<()> {
	file.write(&self.bg_block_bitmap.to_le_bytes())?;
	file.write(&self.bg_inode_bitmap.to_le_bytes())?;
	file.write(&self.bg_inode_table.to_le_bytes())?;
	file.write(&self.bg_free_blocks_count.to_le_bytes())?;
	file.write(&self.bg_free_inodes_count.to_le_bytes())?;
	file.write(&self.bg_used_dirs_count.to_le_bytes())?;
	file.seek(std::io::SeekFrom::Current(BGD_PADDING))?;
	Ok(())
    }
    pub fn new(sb: &Superblock, id: u32) -> Self {
	let mut bgd : BGD = Default::default();
	let num_groups = (sb.s_blocks_count + 511) / sb.s_blocks_per_group;
	bgd.bg_block_bitmap = sb.s_first_data_block + sb.s_blocks_per_group * id + 2;
	bgd.bg_inode_bitmap = bgd.bg_block_bitmap + 1;
	bgd.bg_inode_table = bgd.bg_inode_bitmap + 1;
	if id == num_groups - 1 {
	    bgd.bg_free_blocks_count = (sb.s_blocks_count - sb.s_blocks_per_group * (num_groups - 1)).try_into().unwrap();
	} else {
	    bgd.bg_free_blocks_count = sb.s_blocks_per_group.try_into().unwrap();
	}
	bgd.bg_free_inodes_count = sb.s_inodes_per_group.try_into().unwrap();
	bgd.idx = id;
	bgd
    }
    pub fn get_start(&self, sb : &Superblock) -> u32 {
	self.idx * sb.s_blocks_per_group + sb.s_first_data_block
    }
}
