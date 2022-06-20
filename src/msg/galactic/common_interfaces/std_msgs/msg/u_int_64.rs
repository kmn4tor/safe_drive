// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__UInt64__init(msg: *mut UInt64) -> bool;
    fn std_msgs__msg__UInt64__fini(msg: *mut UInt64);
    fn std_msgs__msg__UInt64__Sequence__init(msg: *mut UInt64SeqRaw, size: usize) -> bool;
    fn std_msgs__msg__UInt64__Sequence__fini(msg: *mut UInt64SeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt64() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct UInt64 {
    pub data: u64,
}

impl UInt64 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__UInt64__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for UInt64 {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__UInt64__fini(self) };
    }
}


struct UInt64SeqRaw {
    data: *mut UInt64,
    size: usize,
    capacity: usize,
}

/// Sequence of UInt64.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct UInt64Seq<const N: usize> {
    data: *mut UInt64,
    size: usize,
    capacity: usize,
}

impl<const N: usize> UInt64Seq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: UInt64SeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__UInt64__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[UInt64]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [UInt64]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for UInt64Seq<N> {
    fn drop(&mut self) {
        let mut msg = UInt64SeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { std_msgs__msg__UInt64__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for UInt64Seq<N> {}
unsafe impl<const N: usize> Sync for UInt64Seq<N> {}


impl TopicMsg for UInt64 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__UInt64()
        }
    }
}
