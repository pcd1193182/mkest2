use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
pub struct Inode {
    pub i_mode : u16,
    pub i_uid : u16,
    pub i_size : u32,
    pub i_atime : u32,
    pub i_ctime : u32,
    pub i_mtime : u32,
    pub i_dtime : u32,
    pub i_gid : u16,
    pub i_links_count : u16,
    pub i_blocks : u32,
    pub i_flags : u32,
    pub i_osd1 : u32,
    pub i_block : [u32; 15],
    pub i_generation : u32,
    pub i_file_acl : u32,
    pub i_dir_acl : u32,
    pub i_faddr : u32,
    pub l_i_frag : u8,
    pub l_i_fsize : u8,
    pub reserved : u16,
    pub l_i_uid_high : u16,
    pub l_i_gid_high : u16
}

pub const INODE_PADDING : i64 = 4;
pub const INODE_SIZE : u32 = 128;

impl Inode {
    pub fn write(&self, mut file : &File) -> std::io::Result<()> {
	file.write(&self.i_mode.to_le_bytes())?;
	file.write(&self.i_uid.to_le_bytes())?;
	file.write(&self.i_size.to_le_bytes())?;
	file.write(&self.i_atime.to_le_bytes())?;
	file.write(&self.i_ctime.to_le_bytes())?;
	file.write(&self.i_mtime.to_le_bytes())?;
	file.write(&self.i_dtime.to_le_bytes())?;
	file.write(&self.i_gid.to_le_bytes())?;
	file.write(&self.i_links_count.to_le_bytes())?;
	file.write(&self.i_blocks.to_le_bytes())?;
	file.write(&self.i_flags.to_le_bytes())?;
	file.write(&self.i_osd1.to_le_bytes())?;
	for x in &self.i_block {
	    file.write(&x.to_le_bytes())?;
	}
	file.write(&self.i_generation.to_le_bytes())?;
	file.write(&self.i_file_acl.to_le_bytes())?;
	file.write(&self.i_dir_acl.to_le_bytes())?;
	file.write(&self.i_faddr.to_le_bytes())?;
	file.write(&self.l_i_frag.to_le_bytes())?;
	file.write(&self.l_i_fsize.to_le_bytes())?;
	file.write(&self.reserved.to_le_bytes())?;
	file.write(&self.l_i_uid_high.to_le_bytes())?;
	file.write(&self.l_i_gid_high.to_le_bytes())?;
	file.seek(std::io::SeekFrom::Current(INODE_PADDING))?;
	Ok(())
    }
    pub fn new(dir : bool) -> Self {
	let mut ret : Inode = Default::default();
	if dir {
	    ret.i_mode = 0x41fd;
	    ret.i_links_count = 2;
	} else {
	    ret.i_mode = 0x1b5;
	    ret.i_links_count = 1;
	}
	ret
    }
}

impl Default for Inode {
    fn default() -> Inode {
	Inode {
	    i_mode: 0,
	    i_uid: 0,
	    i_size: 0,
	    i_atime: 0,
	    i_ctime: 0,
	    i_mtime: 0,
	    i_dtime: 0,
	    i_gid: 0,
	    i_links_count: 0,
	    i_blocks: 0,
	    i_flags: 0,
	    i_osd1: 0,
	    i_block: [0; 15],
	    i_generation: 0,
	    i_file_acl: 0,
	    i_dir_acl: 0,
	    i_faddr: 0,
	    l_i_frag: 0,
	    l_i_fsize: 0,
	    reserved: 0,
	    l_i_uid_high: 0,
	    l_i_gid_high: 0
	}
    }
}
