// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Polygon__init(msg: *mut Polygon) -> bool;
    fn geometry_msgs__msg__Polygon__fini(msg: *mut Polygon);
    fn geometry_msgs__msg__Polygon__Sequence__init(msg: *mut PolygonSeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__Polygon__Sequence__fini(msg: *mut PolygonSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Polygon() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Polygon {
    pub points: Point32Seq<0>,
}

impl Polygon {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Polygon__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Polygon {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Polygon__fini(self) };
    }
}


struct PolygonSeqRaw {
    data: *mut Polygon,
    size: usize,
    capacity: usize,
}

/// Sequence of Polygon.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct PolygonSeq<const N: usize> {
    data: *mut Polygon,
    size: usize,
    capacity: usize,
}

impl<const N: usize> PolygonSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: PolygonSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Polygon__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Polygon]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Polygon]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for PolygonSeq<N> {
    fn drop(&mut self) {
        let mut msg = PolygonSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { geometry_msgs__msg__Polygon__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for PolygonSeq<N> {}
unsafe impl<const N: usize> Sync for PolygonSeq<N> {}


impl TopicMsg for Polygon {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Polygon()
        }
    }
}
