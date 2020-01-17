use std::io::prelude::*;
use std::fs::File;
use uuid::Uuid;
use std::convert::TryInto;

use crate::ext2::BLOCK_SIZE;
use crate::ext2::SECTOR_SIZE;

#[derive(Debug)]
pub struct Superblock {
    pub s_inodes_count : u32,
    pub s_blocks_count : u32,
    pub s_r_blocks_count : u32,
    pub s_free_blocks_count : u32,
    pub s_free_inodes_count : u32,
    pub s_first_data_block : u32,
    pub s_log_block_size : u32,
    pub s_log_frag_size : u32,
    pub s_blocks_per_group : u32,
    pub s_frags_per_group : u32,
    pub s_inodes_per_group : u32,
    pub s_mtime : u32,
    pub s_wtime : u32,
    pub s_mnt_count : u16,
    pub s_max_mnt_count : u16,
    pub s_magic : u16,
    pub s_state : u16,
    pub s_errors : u16,
    pub s_minor_rev_level : u16,
    pub s_lastcheck : u32,
    pub s_checkinterval : u32,
    pub s_creator_os : u32,
    pub s_rev_level : u32,
    pub s_def_resuid : u16,
    pub s_def_resgid : u16,
    // EXT2_DYNAMIC_REV Specific
    pub s_first_ino : u32,
    pub s_inode_size : u16,
    pub s_block_group_nr : u16,
    pub s_feature_compat : u32,
    pub s_feature_incompat : u32,
    pub s_feature_ro_compat : u32,
    pub s_uuid : u128,
    pub s_volume_name : u128,
    pub s_last_mounted : [u64; 8],
    pub s_algo_bitmap : u32,
    // Performance Hints
    pub s_prealloc_blocks : u8,
    pub s_prealloc_dir_blocks : u8,
    alignment : u16,
    // Journaling Support
    pub s_journal_uuid : u128,
    pub s_journal_inum : u32,
    pub s_journal_dev : u32,
    pub s_last_orphan : u32,
    // DIrectory Indexing Support
    pub s_hash_seed : [u64; 4],
    pub s_def_hash_version : u8,
    padding : [u8; 3],
    // Other options
    pub s_default_mount_options : u32,
    pub s_first_meta_bg : u32
}
const SUPERBLOCK_SIZE : u64 = 1024;
pub const SUPERBLOCK_START : u64 = 1024;

impl Superblock {
    pub fn write(&self, mut file : &File) -> std::io::Result<()> {
	let start = file.seek(std::io::SeekFrom::Current(0)).ok().unwrap();
	println!("{}", start);
	file.write(&self.s_inodes_count.to_le_bytes())?;
	file.write(&self.s_blocks_count.to_le_bytes())?;
	file.write(&self.s_r_blocks_count.to_le_bytes())?;
	file.write(&self.s_free_blocks_count.to_le_bytes())?;
	file.write(&self.s_free_inodes_count.to_le_bytes())?;
	file.write(&self.s_first_data_block.to_le_bytes())?;
	file.write(&self.s_log_block_size.to_le_bytes())?;
	file.write(&self.s_log_frag_size.to_le_bytes())?;
	file.write(&self.s_blocks_per_group.to_le_bytes())?;
	file.write(&self.s_frags_per_group.to_le_bytes())?;
	file.write(&self.s_inodes_per_group.to_le_bytes())?;
	file.write(&self.s_mtime.to_le_bytes())?;
	file.write(&self.s_wtime.to_le_bytes())?;
	file.write(&self.s_mnt_count.to_le_bytes())?;
	file.write(&self.s_max_mnt_count.to_le_bytes())?;
	file.write(&self.s_magic.to_le_bytes())?;
	file.write(&self.s_state.to_le_bytes())?;
	file.write(&self.s_errors.to_le_bytes())?;
	file.write(&self.s_minor_rev_level.to_le_bytes())?;
	file.write(&self.s_lastcheck.to_le_bytes())?;
	file.write(&self.s_checkinterval.to_le_bytes())?;
	file.write(&self.s_creator_os.to_le_bytes())?;
	file.write(&self.s_rev_level.to_le_bytes())?;
	file.write(&self.s_def_resuid.to_le_bytes())?;
	file.write(&self.s_def_resgid.to_le_bytes())?;
	file.write(&self.s_first_ino.to_le_bytes())?;
	file.write(&self.s_inode_size.to_le_bytes())?;
	file.write(&self.s_block_group_nr.to_le_bytes())?;
	file.write(&self.s_feature_compat.to_le_bytes())?;
	file.write(&self.s_feature_incompat.to_le_bytes())?;
	file.write(&self.s_feature_ro_compat.to_le_bytes())?;
	file.write(&self.s_uuid.to_le_bytes())?;
	file.write(&self.s_volume_name.to_le_bytes())?;
	for x in &self.s_last_mounted {
	    file.write(&x.to_le_bytes())?;
	}
	file.write(&self.s_algo_bitmap.to_le_bytes())?;
	file.write(&self.s_prealloc_blocks.to_le_bytes())?;
	file.write(&self.s_prealloc_dir_blocks.to_le_bytes())?;
	file.write(&self.alignment.to_le_bytes())?;
	file.write(&self.s_journal_uuid.to_le_bytes())?;
	file.write(&self.s_journal_inum.to_le_bytes())?;
	file.write(&self.s_journal_dev.to_le_bytes())?;
	file.write(&self.s_last_orphan.to_le_bytes())?;
	for x in &self.s_hash_seed {
	    file.write(&x.to_le_bytes())?;
	}
	file.write(&self.s_hash_seed[1].to_le_bytes())?;
	file.write(&self.s_hash_seed[2].to_le_bytes())?;
	file.write(&self.s_hash_seed[3].to_le_bytes())?;
	file.write(&self.s_def_hash_version.to_le_bytes())?;
	file.write(&self.padding)?;
	file.write(&self.s_default_mount_options.to_le_bytes())?;
	file.write(&self.s_first_meta_bg.to_le_bytes())?;
	let current = file.seek(std::io::SeekFrom::Current(0)).ok().unwrap();
	file.seek(std::io::SeekFrom::Current((SUPERBLOCK_SIZE - (current - start)).try_into().unwrap()))?;
	println!("{}", start);
	Ok(())
    }

    pub fn new(num_sectors : u32, num_reserved_sectors : u32) -> Self {
	let mut sb : Superblock = Default::default();
	sb.s_blocks_count = num_sectors * SECTOR_SIZE / BLOCK_SIZE;
	sb.s_r_blocks_count = (num_sectors - num_reserved_sectors) * SECTOR_SIZE / (BLOCK_SIZE * 20);
	sb.s_free_blocks_count = sb.s_blocks_count - 2;
	let num_groups = (sb.s_blocks_count + sb.s_blocks_per_group - 1) / sb.s_blocks_per_group;
	sb.s_inodes_count = sb.s_inodes_per_group * num_groups;
	sb.s_free_inodes_count = sb.s_inodes_count;
	sb.s_uuid = Uuid::new_v4().as_u128();
	sb
    }
}

impl Default for Superblock {
    fn default() -> Superblock {
	Superblock {
	    s_inodes_count: 0,
	    s_blocks_count: 0,
	    s_r_blocks_count: 0,
	    s_free_blocks_count: 0,
	    s_free_inodes_count: 0,
	    s_first_data_block: 1,
	    s_log_block_size: 0,
	    s_log_frag_size: 0,
	    s_blocks_per_group: 513,
	    s_frags_per_group: 513,
	    s_inodes_per_group: 24, // If the first inode table starts at block 6
	    			    // and we only have 8 blocks, we can only have
	    			    // 3 blocks of inode table. At 8 inodes per block,
	    			    // that's 24 inodes per group. This could be increased 
	    			    // by moving the inode table after the reserved sectors.
	    s_mtime: 0,
	    s_wtime: 0,
	    s_mnt_count: 0,
	    s_max_mnt_count: 1024,
	    s_magic: 0xef53,
	    s_state: 1,
	    s_errors: 1,
	    s_minor_rev_level: 0,
	    s_lastcheck: 0,
	    s_checkinterval: 0xbaa8205e,
	    s_creator_os: 0,
	    s_rev_level: 0,
	    s_def_resuid: 0,
	    s_def_resgid: 0,
	    // EXT2_DYNAMIC_REV Specific
	    s_first_ino: 11,
	    s_inode_size: 128,
	    s_block_group_nr: 0,
	    s_feature_compat: 0,
	    s_feature_incompat: 0,
	    s_feature_ro_compat: 0,
	    s_uuid: 0,
	    s_volume_name: 0,
	    s_last_mounted: [0; 8],
	    s_algo_bitmap: 0,
	    // Performance Hints
	    s_prealloc_blocks: 2,
	    s_prealloc_dir_blocks: 0,
	    alignment: 0,
	    // Journaling Support
	    s_journal_uuid: 0,
	    s_journal_inum: 0,
	    s_journal_dev: 0,
	    s_last_orphan: 0,
	    // DIrectory Indexing Support
	    s_hash_seed: [0; 4],
	    s_def_hash_version: 0,
	    padding: [0; 3],
	    // Other options
	    s_default_mount_options: 0,
	    s_first_meta_bg: 0,
	}
    }
}
