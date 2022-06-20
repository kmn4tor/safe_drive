// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__Float32__init(msg: *mut Float32) -> bool;
    fn std_msgs__msg__Float32__fini(msg: *mut Float32);
    fn std_msgs__msg__Float32__Sequence__init(msg: *mut Float32SeqRaw, size: usize) -> bool;
    fn std_msgs__msg__Float32__Sequence__fini(msg: *mut Float32SeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Float32() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Float32 {
    pub data: f32,
}

impl Float32 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Float32__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Float32 {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Float32__fini(self) };
    }
}


struct Float32SeqRaw {
    data: *mut Float32,
    size: usize,
    capacity: usize,
}

/// Sequence of Float32.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct Float32Seq<const N: usize> {
    data: *mut Float32,
    size: usize,
    capacity: usize,
}

impl<const N: usize> Float32Seq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: Float32SeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Float32__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Float32]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Float32]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for Float32Seq<N> {
    fn drop(&mut self) {
        let mut msg = Float32SeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_msgs__msg__Float32__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for Float32Seq<N> {}
unsafe impl<const N: usize> Sync for Float32Seq<N> {}


impl TopicMsg for Float32 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Float32()
        }
    }
}
