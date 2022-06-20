// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn diagnostic_msgs__srv__SelfTest_Request__init(msg: *mut SelfTestRequest) -> bool;
    fn diagnostic_msgs__srv__SelfTest_Request__fini(msg: *mut SelfTestRequest);
    fn diagnostic_msgs__srv__SelfTest_Request__Sequence__init(msg: *mut SelfTestRequestSeqRaw, size: usize) -> bool;
    fn diagnostic_msgs__srv__SelfTest_Request__Sequence__fini(msg: *mut SelfTestRequestSeqRaw);
    fn diagnostic_msgs__srv__SelfTest_Response__init(msg: *mut SelfTestResponse) -> bool;
    fn diagnostic_msgs__srv__SelfTest_Response__fini(msg: *mut SelfTestResponse);
    fn diagnostic_msgs__srv__SelfTest_Response__Sequence__init(msg: *mut SelfTestResponseSeqRaw, size: usize) -> bool;
    fn diagnostic_msgs__srv__SelfTest_Response__Sequence__fini(msg: *mut SelfTestResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__diagnostic_msgs__srv__SelfTest() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct SelfTestRequest {
    _unused: u8
}

#[repr(C)]
#[derive(Debug)]
pub struct SelfTestResponse {
    pub id: crate::msg::RosString<0>,
    pub passed: u8,
    pub status: DiagnosticStatusSeq<0>,
}

impl SelfTestRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__SelfTest_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SelfTestRequest {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__srv__SelfTest_Request__fini(self) };
    }
}


struct SelfTestRequestSeqRaw {
    data: *mut SelfTestRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of SelfTestRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SelfTestRequestSeq<const N: usize> {
    data: *mut SelfTestRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SelfTestRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SelfTestRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__SelfTest_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[SelfTestRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [SelfTestRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for SelfTestRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = SelfTestRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { diagnostic_msgs__srv__SelfTest_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SelfTestRequestSeq<N> {}
unsafe impl<const N: usize> Sync for SelfTestRequestSeq<N> {}


impl SelfTestResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__SelfTest_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SelfTestResponse {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__srv__SelfTest_Response__fini(self) };
    }
}


struct SelfTestResponseSeqRaw {
    data: *mut SelfTestResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of SelfTestResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct SelfTestResponseSeq<const N: usize> {
    data: *mut SelfTestResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> SelfTestResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: SelfTestResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__SelfTest_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[SelfTestResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [SelfTestResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for SelfTestResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = SelfTestResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { diagnostic_msgs__srv__SelfTest_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for SelfTestResponseSeq<N> {}
unsafe impl<const N: usize> Sync for SelfTestResponseSeq<N> {}


pub struct SelfTest;

impl ServiceMsg for SelfTest {
    type Request = SelfTestRequest;
    type Response = SelfTestResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__diagnostic_msgs__srv__SelfTest()
        }
    }
}

