// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn diagnostic_msgs__srv__AddDiagnostics_Request__init(msg: *mut AddDiagnosticsRequest) -> bool;
    fn diagnostic_msgs__srv__AddDiagnostics_Request__fini(msg: *mut AddDiagnosticsRequest);
    fn diagnostic_msgs__srv__AddDiagnostics_Request__Sequence__init(msg: *mut AddDiagnosticsRequestSeqRaw, size: usize) -> bool;
    fn diagnostic_msgs__srv__AddDiagnostics_Request__Sequence__fini(msg: *mut AddDiagnosticsRequestSeqRaw);
    fn diagnostic_msgs__srv__AddDiagnostics_Response__init(msg: *mut AddDiagnosticsResponse) -> bool;
    fn diagnostic_msgs__srv__AddDiagnostics_Response__fini(msg: *mut AddDiagnosticsResponse);
    fn diagnostic_msgs__srv__AddDiagnostics_Response__Sequence__init(msg: *mut AddDiagnosticsResponseSeqRaw, size: usize) -> bool;
    fn diagnostic_msgs__srv__AddDiagnostics_Response__Sequence__fini(msg: *mut AddDiagnosticsResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__diagnostic_msgs__srv__AddDiagnostics() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct AddDiagnosticsRequest {
    pub load_namespace: crate::msg::RosString<0>,
}

#[repr(C)]
#[derive(Debug)]
pub struct AddDiagnosticsResponse {
    pub success: bool,
    pub message: crate::msg::RosString<0>,
}

impl AddDiagnosticsRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for AddDiagnosticsRequest {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__fini(self) };
    }
}


struct AddDiagnosticsRequestSeqRaw {
    data: *mut AddDiagnosticsRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of AddDiagnosticsRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct AddDiagnosticsRequestSeq<const N: usize> {
    data: *mut AddDiagnosticsRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> AddDiagnosticsRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: AddDiagnosticsRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[AddDiagnosticsRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [AddDiagnosticsRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for AddDiagnosticsRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = AddDiagnosticsRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { diagnostic_msgs__srv__AddDiagnostics_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for AddDiagnosticsRequestSeq<N> {}
unsafe impl<const N: usize> Sync for AddDiagnosticsRequestSeq<N> {}


impl AddDiagnosticsResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for AddDiagnosticsResponse {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__fini(self) };
    }
}


struct AddDiagnosticsResponseSeqRaw {
    data: *mut AddDiagnosticsResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of AddDiagnosticsResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct AddDiagnosticsResponseSeq<const N: usize> {
    data: *mut AddDiagnosticsResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> AddDiagnosticsResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: AddDiagnosticsResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[AddDiagnosticsResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [AddDiagnosticsResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for AddDiagnosticsResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = AddDiagnosticsResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { diagnostic_msgs__srv__AddDiagnostics_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for AddDiagnosticsResponseSeq<N> {}
unsafe impl<const N: usize> Sync for AddDiagnosticsResponseSeq<N> {}


pub struct AddDiagnostics;

impl ServiceMsg for AddDiagnostics {
    type Request = AddDiagnosticsRequest;
    type Response = AddDiagnosticsResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__diagnostic_msgs__srv__AddDiagnostics()
        }
    }
}

