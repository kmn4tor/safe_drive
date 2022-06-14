// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
use crate::msg::common_interfaces::*;
pub const RESULT_SUCCESS: u8 = 0;
pub const RESULT_MAP_DOES_NOT_EXIST: u8 = 1;
pub const RESULT_INVALID_MAP_DATA: u8 = 2;
pub const RESULT_INVALID_MAP_METADATA: u8 = 3;
pub const RESULT_UNDEFINED_FAILURE: u8 = 255;

extern "C" {
    fn nav_msgs__srv__LoadMap_Request__init(msg: *mut LoadMapRequest) -> bool;
    fn nav_msgs__srv__LoadMap_Request__fini(msg: *mut LoadMapRequest);
    fn nav_msgs__srv__LoadMap_Request__Sequence__init(msg: *mut LoadMapRequestSequence, size: usize) -> bool;
    fn nav_msgs__srv__LoadMap_Request__Sequence__fini(msg: *mut LoadMapRequestSequence);
    fn nav_msgs__srv__LoadMap_Response__init(msg: *mut LoadMapResponse) -> bool;
    fn nav_msgs__srv__LoadMap_Response__fini(msg: *mut LoadMapResponse);
    fn nav_msgs__srv__LoadMap_Response__Sequence__init(msg: *mut LoadMapResponseSequence, size: usize) -> bool;
    fn nav_msgs__srv__LoadMap_Response__Sequence__fini(msg: *mut LoadMapResponseSequence);
    fn rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__LoadMap() -> *const rcl::rosidl_service_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct LoadMapRequest {
    pub map_url: crate::msg::RosString<0>,
}

#[repr(C)]
#[derive(Debug)]
pub struct LoadMapResponse {
    pub map: OccupancyGrid,
    pub result: u8,
}

impl LoadMapRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__LoadMap_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for LoadMapRequest {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__LoadMap_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LoadMapRequestSequence {
    data: *mut LoadMapRequest,
    size: usize,
    capacity: usize,
}

impl LoadMapRequestSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__LoadMap_Request__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[LoadMapRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [LoadMapRequest]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for LoadMapRequestSequence {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__LoadMap_Request__Sequence__fini(self) };
    }
}

impl LoadMapResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__LoadMap_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for LoadMapResponse {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__LoadMap_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LoadMapResponseSequence {
    data: *mut LoadMapResponse,
    size: usize,
    capacity: usize,
}

impl LoadMapResponseSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__srv__LoadMap_Response__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[LoadMapResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [LoadMapResponse]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for LoadMapResponseSequence {
    fn drop(&mut self) {
        unsafe { nav_msgs__srv__LoadMap_Response__Sequence__fini(self) };
    }
}

pub struct LoadMap;

impl ServiceMsg for LoadMap {
    type Request = LoadMapRequest;
    type Response = LoadMapResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__LoadMap()
        }
    }
}
