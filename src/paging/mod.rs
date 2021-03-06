//! Abstractions for page tables and other paging related structures.
//!
//! Page tables translate virtual memory “pages” to physical memory “frames”.

#![allow(non_upper_case_globals)]

pub use self::{
    frame::PhysFrame,
    frame_alloc::{FrameAllocator, FrameDeallocator},
};

pub use self::mapper::{MappedPageTable, Mapper, RecursivePageTable};

pub use self::{
    page::{Page, PageSize, Size1GiB, Size2MiB, Size4KiB},
    page_table::{PageTable, PageTableAttribute, PageTableEntry, PageTableFlags},
};

pub mod frame;
mod frame_alloc;
pub mod mapper;
pub mod memory_attribute;
pub mod page;
pub mod page_table;
