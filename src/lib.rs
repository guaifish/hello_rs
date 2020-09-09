use core::ops::Deref;

#[repr(C)]
pub struct Task {
    index: usize,
    // other private fields
}

impl Task {
    pub fn new(index: usize) -> Task {
        Task { index }
    }
}

#[repr(C)]
pub struct ReadOnlyTask {
    pub index: usize,
    // the same private fields
}

impl Deref for Task {
    type Target = ReadOnlyTask;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}