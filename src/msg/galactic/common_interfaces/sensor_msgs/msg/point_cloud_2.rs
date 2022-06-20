// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__PointCloud2__init(msg: *mut PointCloud2) -> bool;
    fn sensor_msgs__msg__PointCloud2__fini(msg: *mut PointCloud2);
    fn sensor_msgs__msg__PointCloud2__Sequence__init(msg: *mut PointCloud2SeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__PointCloud2__Sequence__fini(msg: *mut PointCloud2SeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointCloud2() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct PointCloud2 {
    pub header: std_msgs::msg::Header,
    pub height: u32,
    pub width: u32,
    pub fields: PointFieldSeq<0>,
    pub is_bigendian: bool,
    pub point_step: u32,
    pub row_step: u32,
    pub data: crate::msg::U8Seq<0>,
    pub is_dense: bool,
}

impl PointCloud2 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__PointCloud2__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for PointCloud2 {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__PointCloud2__fini(self) };
    }
}


struct PointCloud2SeqRaw {
    data: *mut PointCloud2,
    size: usize,
    capacity: usize,
}

/// Sequence of PointCloud2.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PointCloud2Seq<const N: usize> {
    data: *mut PointCloud2,
    size: usize,
    capacity: usize,
}

impl<const N: usize> PointCloud2Seq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PointCloud2SeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__PointCloud2__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[PointCloud2]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [PointCloud2]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for PointCloud2Seq<N> {
    fn drop(&mut self) {
        let mut msg = PointCloud2SeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__PointCloud2__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PointCloud2Seq<N> {}
unsafe impl<const N: usize> Sync for PointCloud2Seq<N> {}


impl TopicMsg for PointCloud2 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__PointCloud2()
        }
    }
}
