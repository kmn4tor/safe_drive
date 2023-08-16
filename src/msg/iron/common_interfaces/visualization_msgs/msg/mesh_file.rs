// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn visualization_msgs__msg__MeshFile__init(msg: *mut MeshFile) -> bool;
    fn visualization_msgs__msg__MeshFile__fini(msg: *mut MeshFile);
    fn visualization_msgs__msg__MeshFile__are_equal(lhs: *const MeshFile, rhs: *const MeshFile) -> bool;
    fn visualization_msgs__msg__MeshFile__Sequence__init(msg: *mut MeshFileSeqRaw, size: usize) -> bool;
    fn visualization_msgs__msg__MeshFile__Sequence__fini(msg: *mut MeshFileSeqRaw);
    fn visualization_msgs__msg__MeshFile__Sequence__are_equal(lhs: *const MeshFileSeqRaw, rhs: *const MeshFileSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__MeshFile() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct MeshFile {
    pub filename: crate::msg::RosString<0>,
    pub data: crate::msg::U8Seq<0>,
}

impl MeshFile {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__MeshFile__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MeshFile {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__MeshFile__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MeshFileSeqRaw {
    data: *mut MeshFile,
    size: usize,
    capacity: usize,
}

/// Sequence of MeshFile.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MeshFileSeq<const N: usize> {
    data: *mut MeshFile,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MeshFileSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: MeshFileSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__MeshFile__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: MeshFileSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[MeshFile] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MeshFile] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MeshFile> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MeshFile> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MeshFileSeq<N> {
    fn drop(&mut self) {
        let mut msg = MeshFileSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { visualization_msgs__msg__MeshFile__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MeshFileSeq<N> {}
unsafe impl<const N: usize> Sync for MeshFileSeq<N> {}


impl TypeSupport for MeshFile {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__visualization_msgs__msg__MeshFile()
        }
    }
}

impl PartialEq for MeshFile {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            visualization_msgs__msg__MeshFile__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for MeshFileSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MeshFileSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = MeshFileSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            visualization_msgs__msg__MeshFile__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

