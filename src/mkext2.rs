use std::env;
use std::process;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::Seek;
use std::convert::TryInto;

mod ext2;
use ext2::SECTOR_SIZE;
use ext2::BLOCK_SIZE;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
	println!("Not enough arguments: {} <img file name> <Total size in sectors> <Number of reserved sectors>", &args[0]);
	process::exit(1);
    }
    let num_sectors : u32 = match args[2].parse() {
	Result::Err(_) => {
	    println!("Invalid number of sectors: {}", &args[2]);
	    process::exit(2);
	},
	Ok(i) => i
    };
    let num_sectors_res : u32 = match args[3].parse() {
	Result::Err(_) => {
	    println!("Invalid number of reserved sectors: {}", &args[3]);
	    process::exit(3);
	},
	Ok(i) => i
    };
    let filename = &args[1];

    let mut file = match File::create(filename) {
	IOResult::Err(e) => {
	    println!("IO Error when creating {}: {}", filename, e);
	    process::exit(4);
	},
	IOResult::Ok(f) => f
    };
    println!("{} {} {}", num_sectors, num_sectors_res, filename);

    let res = file.set_len((num_sectors * SECTOR_SIZE).into());
    if !res.is_ok() {
	println!("IO Error when extending file: {}", res.err().unwrap());
	process::exit(5);
    }
    let mut sb = ext2::superblock::Superblock::new(num_sectors, num_sectors_res);
    if num_sectors_res * SECTOR_SIZE / BLOCK_SIZE >= sb.s_blocks_per_group + 1 {
	println!("Too many reserved sectors!");
	process::exit(16);
    }
    let mut bgd_reserved = ext2::bgd::BGD::new(&sb, 0);
    let num_groups = (sb.s_blocks_count + sb.s_blocks_per_group - 1) / sb.s_blocks_per_group;
    let mut bgds : Vec<ext2::bgd::BGD> = Vec::new();
    for i in 1 .. num_groups {
	bgds.push(ext2::bgd::BGD::new(&sb, i));
    }

    let used_blk_id = bgd_reserved.bg_inode_table + (sb.s_inodes_per_group * ext2::inode::INODE_SIZE / BLOCK_SIZE);
    let mut block_bmap_reserved = ext2::bitmap::Bitmap::new(sb.s_blocks_per_group);
    for i in 0 .. used_blk_id {
	block_bmap_reserved.set(i, true);
	sb.s_free_blocks_count -= 1;
	bgd_reserved.bg_free_blocks_count -= 1;
    }

    if num_sectors_res > used_blk_id {
	for i in used_blk_id .. (num_sectors_res * SECTOR_SIZE / BLOCK_SIZE) - 1 {
	    block_bmap_reserved.set(i, true);
	    sb.s_free_blocks_count -= 1;
	    bgd_reserved.bg_free_blocks_count -= 1;
	}
    }

    let mut inode_bmap_reserved = ext2::bitmap::Bitmap::new(sb.s_inodes_per_group);
    for i in 0 .. sb.s_first_ino {
	inode_bmap_reserved.set(i, true);
	sb.s_free_inodes_count -= 1;
	bgd_reserved.bg_free_inodes_count -= 1;
    }

    let mut block_bmaps : Vec<ext2::bitmap::Bitmap> = Vec::new();
    let mut inode_bmaps : Vec<ext2::bitmap::Bitmap> = Vec::new();
    for i in 0 .. num_groups - 1 {
	let mut bgd = &mut bgds[i as usize];
	let mut block_bmap = ext2::bitmap::Bitmap::new(sb.s_blocks_per_group);
	for j in 0 .. bgd.bg_inode_table - bgd.get_start(&sb) + (sb.s_inodes_per_group * ext2::inode::INODE_SIZE / BLOCK_SIZE) {
	    block_bmap.set(j, true);
	    sb.s_free_blocks_count -= 1;
	    bgd.bg_free_blocks_count -= 1;
	}
	
	let inode_bmap = ext2::bitmap::Bitmap::new(sb.s_inodes_per_group);
	block_bmaps.push(block_bmap);
	inode_bmaps.push(inode_bmap);
    }

    let mut inodes : Vec<ext2::inode::Inode> = Vec::new();
//    inodes.push(ext2::inode::Inode::new(false)); // empty inode
    inodes.push(Default::default()); // Bad Blocks inode
    let mut root_inode = ext2::inode::Inode::new(true);
    root_inode.i_block[0] = block_bmap_reserved.alloc(&mut bgd_reserved, &mut sb).unwrap();
    root_inode.i_blocks += BLOCK_SIZE / SECTOR_SIZE;

    let root_dir : ext2::directory::Directory = ext2::directory::Directory::new(2, 2);
    root_inode.i_size = 1024;//root_dir.len();

    inodes.push(root_inode); // Root Directory inode
    inodes.push(Default::default()); // ACL index inode
    inodes.push(Default::default()); // ACL data inode
    inodes.push(ext2::inode::Inode::new(false)); // boot loader inode
    inodes.push(ext2::inode::Inode::new(true)); // undelete directory inode
    bgd_reserved.bg_used_dirs_count += 2;


    let res = file.seek(std::io::SeekFrom::Start(ext2::superblock::SUPERBLOCK_START));
    if !res.is_ok() {
	println!("IO Error when seeking to superblock: {}", res.err().unwrap());
	process::exit(20);
    }
    println!("{:#?}", sb);
    let res = sb.write(&file);
    if !res.is_ok() {
	println!("IO Error when writing superblock: {}", res.err().unwrap());
	process::exit(6);
    }
    println!("{:#?}", bgd_reserved);
    let res = bgd_reserved.write(&file);
    if !res.is_ok() {
	println!("IO Error when writing reserved bgd: {}", res.err().unwrap());
	process::exit(7);
    }
    for i in 0 .. (num_groups - 1) {
	let bgd = &bgds[i as usize];
	let res = bgd.write(&file);
	if !res.is_ok() {
	    println!("IO Error when writing bgd {}: {}", i + 1, res.err().unwrap());
	    process::exit(15);
	}
    }
    let res = file.seek(std::io::SeekFrom::Current((BLOCK_SIZE - (num_groups * ext2::bgd::BGD_SIZE)).try_into().unwrap()));
    if !res.is_ok() {
	println!("IO Error when seeking to reserved block bitmap: {}", res.err().unwrap());
	process::exit(9);
    }
    println!("bmap: {}", file.seek(std::io::SeekFrom::Current(0)).ok().unwrap());
    let res = block_bmap_reserved.write(&file);
    if !res.is_ok() {
	println!("IO Error when writing reserved block bitmap: {}", res.err().unwrap());
	process::exit(7);
    }
    println!("bmap: {}", file.seek(std::io::SeekFrom::Current(0)).ok().unwrap());
    let res = inode_bmap_reserved.write(&file);
    if !res.is_ok() {
	println!("IO Error when writing reserved inode bitmap: {}", res.err().unwrap());
	process::exit(7);
    }
    for inode in &inodes {
	println!("{:#?}", inode);
	let res = inode.write(&file);
	if !res.is_ok() {
	    println!("IO Error when writing inode table: {}", res.err().unwrap());
	    process::exit(8);
	}
    }
    let res = file.seek(std::io::SeekFrom::Current((((sb.s_inodes_per_group) - inodes.len() as u32) * ext2::inode::INODE_SIZE).try_into().unwrap()));
    if !res.is_ok() {
	println!("IO Error when seeking past inode table: {}", res.err().unwrap());
	process::exit(9);
    }
    for i in 0 .. (num_groups - 1) {
	let bgd = &bgds[i as usize];
	let block_bmap = &block_bmaps[i as usize];
	let inode_bmap = &inode_bmaps[i as usize];

	println!("Writing backup sb {}", i);
	let res = file.seek(std::io::SeekFrom::Start(bgd.get_start(&sb) as u64 * BLOCK_SIZE as u64));
	if !res.is_ok() {
	    println!("IO Error when seeking to bgd #{}: {}", i, res.err().unwrap());
	    process::exit(9);
	}
	println!("at: {}", file.seek(std::io::SeekFrom::Current(0)).ok().unwrap());
	let res = sb.write(&file);
	if !res.is_ok() {
	    println!("IO Error when writing superblock backup {}: {}", i, res.err().unwrap());
	    process::exit(6);
	}
	println!("{:#?}", bgd_reserved);
	let res = bgd_reserved.write(&file);
	if !res.is_ok() {
	    println!("IO Error when writing reserved bgd backup {}: {}", i, res.err().unwrap());
	    process::exit(7);
	}
	for j in 0 .. (num_groups - 1) {
	    let bgd = &bgds[j as usize];
	    let res = bgd.write(&file);
	    if !res.is_ok() {
		println!("IO Error when writing backup bgd {} {}: {}", i, j + 1, res.err().unwrap());
		process::exit(15);
	    }
	}
	println!("at: {}", file.seek(std::io::SeekFrom::Current(0)).ok().unwrap());
	let res = file.seek(std::io::SeekFrom::Current((BLOCK_SIZE - (num_groups * ext2::bgd::BGD_SIZE)).try_into().unwrap()));
	if !res.is_ok() {
	    println!("IO Error when seeking to reserved block bitmap: {}", res.err().unwrap());
	    process::exit(9);
	}
	let res = block_bmap.write(&file);
	if !res.is_ok() {
	    println!("IO Error when writing bitmap {}: {}", i, res.err().unwrap());
	    process::exit(10);
	}
	let res = inode_bmap.write(&file);
	if !res.is_ok() {
	    println!("IO Error when writing inode bitmap {}: {}", i, res.err().unwrap());
	    process::exit(11);
	}
    }

    let res = file.seek(std::io::SeekFrom::Start((BLOCK_SIZE * inodes[1].i_block[0]) as u64));
    if !res.is_ok() {
	println!("IO Error when seeking to root directory: {}", res.err().unwrap());
	process::exit(18);
    }
    let res = root_dir.write(&file);
    if !res.is_ok() {
	println!("IO Error when writing to root directory: {}", res.err().unwrap());
	process::exit(19);
    }
}
