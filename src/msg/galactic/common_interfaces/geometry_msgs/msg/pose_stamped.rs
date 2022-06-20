// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__PoseStamped__init(msg: *mut PoseStamped) -> bool;
    fn geometry_msgs__msg__PoseStamped__fini(msg: *mut PoseStamped);
    fn geometry_msgs__msg__PoseStamped__Sequence__init(msg: *mut PoseStampedSeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__PoseStamped__Sequence__fini(msg: *mut PoseStampedSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PoseStamped() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct PoseStamped {
    pub header: std_msgs::msg::Header,
    pub pose: Pose,
}

impl PoseStamped {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__PoseStamped__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for PoseStamped {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__PoseStamped__fini(self) };
    }
}


struct PoseStampedSeqRaw {
    data: *mut PoseStamped,
    size: usize,
    capacity: usize,
}

/// Sequence of PoseStamped.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PoseStampedSeq<const N: usize> {
    data: *mut PoseStamped,
    size: usize,
    capacity: usize,
}

impl<const N: usize> PoseStampedSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PoseStampedSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__PoseStamped__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[PoseStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [PoseStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for PoseStampedSeq<N> {
    fn drop(&mut self) {
        let mut msg = PoseStampedSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { geometry_msgs__msg__PoseStamped__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PoseStampedSeq<N> {}
unsafe impl<const N: usize> Sync for PoseStampedSeq<N> {}


impl TopicMsg for PoseStamped {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PoseStamped()
        }
    }
}
