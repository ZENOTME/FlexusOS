mod heap_allocator;
use heap_allocator::*;
pub use heap_allocator::heap_test;
pub mod address;
pub mod page;
pub mod frame;

//pub mod address;
pub fn init(){
    init_heap();
}