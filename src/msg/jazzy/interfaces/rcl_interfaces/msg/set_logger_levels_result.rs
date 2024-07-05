// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn rcl_interfaces__msg__SetLoggerLevelsResult__init(msg: *mut SetLoggerLevelsResult) -> bool;
    fn rcl_interfaces__msg__SetLoggerLevelsResult__fini(msg: *mut SetLoggerLevelsResult);
    fn rcl_interfaces__msg__SetLoggerLevelsResult__are_equal(
        lhs: *const SetLoggerLevelsResult,
        rhs: *const SetLoggerLevelsResult,
    ) -> bool;
    fn rcl_interfaces__msg__SetLoggerLevelsResult__Sequence__init(
        msg: *mut SetLoggerLevelsResultSeqRaw,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__SetLoggerLevelsResult__Sequence__fini(
        msg: *mut SetLoggerLevelsResultSeqRaw,
    );
    fn rcl_interfaces__msg__SetLoggerLevelsResult__Sequence__are_equal(
        lhs: *const SetLoggerLevelsResultSeqRaw,
        rhs: *const SetLoggerLevelsResultSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__SetLoggerLevelsResult(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct SetLoggerLevelsResult {
    pub successful: bool,
    pub reason: crate::msg::RosString<0>,
}

impl SetLoggerLevelsResult {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__SetLoggerLevelsResult__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SetLoggerLevelsResult {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__msg__SetLoggerLevelsResult__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct SetLoggerLevelsResultSeqRaw {
    data: *mut SetLoggerLevelsResult,
    size: size_t,
    capacity: size_t,
}

/// Sequence of SetLoggerLevelsResult.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SetLoggerLevelsResultSeq<const N: usize> {
    data: *mut SetLoggerLevelsResult,
    size: size_t,
    capacity: size_t,
}

impl<const N: usize> SetLoggerLevelsResultSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size > N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SetLoggerLevelsResultSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__SetLoggerLevelsResult__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: SetLoggerLevelsResultSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[SetLoggerLevelsResult] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size as _) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [SetLoggerLevelsResult] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size as _) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SetLoggerLevelsResult> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, SetLoggerLevelsResult> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for SetLoggerLevelsResultSeq<N> {
    fn drop(&mut self) {
        let mut msg = SetLoggerLevelsResultSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { rcl_interfaces__msg__SetLoggerLevelsResult__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SetLoggerLevelsResultSeq<N> {}
unsafe impl<const N: usize> Sync for SetLoggerLevelsResultSeq<N> {}

impl TypeSupport for SetLoggerLevelsResult {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__SetLoggerLevelsResult()
        }
    }
}

impl PartialEq for SetLoggerLevelsResult {
    fn eq(&self, other: &Self) -> bool {
        unsafe { rcl_interfaces__msg__SetLoggerLevelsResult__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for SetLoggerLevelsResultSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = SetLoggerLevelsResultSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = SetLoggerLevelsResultSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            rcl_interfaces__msg__SetLoggerLevelsResult__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
