// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn visualization_msgs__msg__InteractiveMarker__init(msg: *mut InteractiveMarker) -> bool;
    fn visualization_msgs__msg__InteractiveMarker__fini(msg: *mut InteractiveMarker);
    fn visualization_msgs__msg__InteractiveMarker__Sequence__init(msg: *mut InteractiveMarkerSeqRaw, size: usize) -> bool;
    fn visualization_msgs__msg__InteractiveMarker__Sequence__fini(msg: *mut InteractiveMarkerSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarker() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarker {
    pub header: std_msgs::msg::Header,
    pub pose: geometry_msgs::msg::Pose,
    pub name: crate::msg::RosString<0>,
    pub description: crate::msg::RosString<0>,
    pub scale: f32,
    pub menu_entries: MenuEntrySeq<0>,
    pub controls: InteractiveMarkerControlSeq<0>,
}

impl InteractiveMarker {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarker__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for InteractiveMarker {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__InteractiveMarker__fini(self) };
    }
}


struct InteractiveMarkerSeqRaw {
    data: *mut InteractiveMarker,
    size: usize,
    capacity: usize,
}

/// Sequence of InteractiveMarker.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerSeq<const N: usize> {
    data: *mut InteractiveMarker,
    size: usize,
    capacity: usize,
}

impl<const N: usize> InteractiveMarkerSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: InteractiveMarkerSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarker__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[InteractiveMarker]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [InteractiveMarker]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for InteractiveMarkerSeq<N> {
    fn drop(&mut self) {
        let mut msg = InteractiveMarkerSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { visualization_msgs__msg__InteractiveMarker__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for InteractiveMarkerSeq<N> {}
unsafe impl<const N: usize> Sync for InteractiveMarkerSeq<N> {}


impl TopicMsg for InteractiveMarker {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__InteractiveMarker()
        }
    }
}
