use super::BlockDevice;
use crate::mm::{
    frame_alloc, frame_dealloc, kernel_token, FrameTracker, PageTable, PhysAddr, PhysPageNum,
    StepByOne, VirtAddr,
};
use crate::sync::UPSafeCell;
use alloc::vec::Vec;
use lazy_static::*;

pub struct RamDiskBlock(UPSafeCell<RamDiskBlk>);

impl BlockDevice for RamDiskBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0
            .exclusive_access()
            .read_block(block_id, buf)
            .expect("Error when reading RamDiskBlk");
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .exclusive_access()
            .write_block(block_id, buf)
            .expect("Error when writing RamDiskBlk");
    }
}

impl RamDiskBlock{
    #[allow(unused)]
    pub fn new() -> Self {
        unsafe {
            Self(UPSafeCell::new(
                RamDiskBlk::new(0x84200000),
            ))
        }
    }
}

pub struct RamDiskBlk {
    diskaddr: usize,
}

impl RamDiskBlk{
    pub fn new(diskaddr: usize) -> Self {
        Self { diskaddr }
    }
    pub fn read_block(&mut self, block_id: usize, buf: &mut [u8]) -> Result<(), &'static str> {

        // println!("read_block block_id: {}", block_id);
            
        let ptr = self.diskaddr + block_id * 512;

        let buff: *mut u8 = (ptr) as *mut _;
        unsafe {
            buf.as_mut_ptr().copy_from_nonoverlapping(buff, 512);
        }
        // println!("buf: {:x?}", buf);
        Ok(())
    }
    pub fn write_block(&mut self, block_id: usize, buf: &[u8]) -> Result<(), &'static str> {
        // println!("write_block block_id: {}", block_id);

        let ptr = self.diskaddr + block_id * 512;

        let buff: *mut u8 = (ptr) as *mut _;

        unsafe {
            buff.copy_from_nonoverlapping(buf.as_ptr(), 512);
        }

        // println!("write from {:x?} to {:#x?}", buf, buff);
        Ok(())
    }
}

