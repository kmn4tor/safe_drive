// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;

extern "C" {
    fn nav_msgs__srv__GetPlan_Request__init(msg: *mut GetPlanRequest) -> bool;
    fn nav_msgs__srv__GetPlan_Request__fini(msg: *mut GetPlanRequest);
    fn nav_msgs__srv__GetPlan_Request__Sequence__init(msg: *mut GetPlanRequestSeqRaw, size: usize) -> bool;
    fn nav_msgs__srv__GetPlan_Request__Sequence__fini(msg: *mut GetPlanRequestSeqRaw);
    fn nav_msgs__srv__GetPlan_Response__init(msg: *mut GetPlanResponse) -> bool;
    fn nav_msgs__srv__GetPlan_Response__fini(msg: *mut GetPlanResponse);
    fn nav_msgs__srv__GetPlan_Response__Sequence__init(msg: *mut GetPlanResponseSeqRaw, size: usize) -> bool;
    fn nav_msgs__srv__GetPlan_Response__Sequence__fini(msg: *mut GetPlanResponseSeqRaw);
    fn rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__GetPlan() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct GetPlanRequest {
    pub start: geometry_msgs::msg::PoseStamped,
    pub goal: geometry_msgs::msg::PoseStamped,
    pub tolerance: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct GetPlanResponse {
    pub plan: Path,
}

impl GetPlanRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__GetPlan_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GetPlanRequest {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__GetPlan_Request__fini(self) };
    }
}


struct GetPlanRequestSeqRaw {
    data: *mut GetPlanRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of GetPlanRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GetPlanRequestSeq<const N: usize> {
    data: *mut GetPlanRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> GetPlanRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GetPlanRequestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__GetPlan_Request__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[GetPlanRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [GetPlanRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for GetPlanRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = GetPlanRequestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { nav_msgs__srv__GetPlan_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for GetPlanRequestSeq<N> {}
unsafe impl<const N: usize> Sync for GetPlanRequestSeq<N> {}


impl GetPlanResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__GetPlan_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GetPlanResponse {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__GetPlan_Response__fini(self) };
    }
}


struct GetPlanResponseSeqRaw {
    data: *mut GetPlanResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of GetPlanResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GetPlanResponseSeq<const N: usize> {
    data: *mut GetPlanResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> GetPlanResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GetPlanResponseSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__GetPlan_Response__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[GetPlanResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [GetPlanResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for GetPlanResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = GetPlanResponseSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { nav_msgs__srv__GetPlan_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for GetPlanResponseSeq<N> {}
unsafe impl<const N: usize> Sync for GetPlanResponseSeq<N> {}


pub struct GetPlan;

impl ServiceMsg for GetPlan {
    type Request = GetPlanRequest;
    type Response = GetPlanResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__GetPlan()
        }
    }
}

