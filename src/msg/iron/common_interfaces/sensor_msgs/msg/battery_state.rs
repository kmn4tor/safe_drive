// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const POWER_SUPPLY_STATUS_UNKNOWN: u8 = 0;
pub const POWER_SUPPLY_STATUS_CHARGING: u8 = 1;
pub const POWER_SUPPLY_STATUS_DISCHARGING: u8 = 2;
pub const POWER_SUPPLY_STATUS_NOT_CHARGING: u8 = 3;
pub const POWER_SUPPLY_STATUS_FULL: u8 = 4;
pub const POWER_SUPPLY_HEALTH_UNKNOWN: u8 = 0;
pub const POWER_SUPPLY_HEALTH_GOOD: u8 = 1;
pub const POWER_SUPPLY_HEALTH_OVERHEAT: u8 = 2;
pub const POWER_SUPPLY_HEALTH_DEAD: u8 = 3;
pub const POWER_SUPPLY_HEALTH_OVERVOLTAGE: u8 = 4;
pub const POWER_SUPPLY_HEALTH_UNSPEC_FAILURE: u8 = 5;
pub const POWER_SUPPLY_HEALTH_COLD: u8 = 6;
pub const POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE: u8 = 7;
pub const POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE: u8 = 8;
pub const POWER_SUPPLY_TECHNOLOGY_UNKNOWN: u8 = 0; // Unknown battery technology
pub const POWER_SUPPLY_TECHNOLOGY_NIMH: u8 = 1; // Nickel-Metal Hydride battery
pub const POWER_SUPPLY_TECHNOLOGY_LION: u8 = 2; // Lithium-ion battery
pub const POWER_SUPPLY_TECHNOLOGY_LIPO: u8 = 3; // Lithium Polymer battery
pub const POWER_SUPPLY_TECHNOLOGY_LIFE: u8 = 4; // Lithium Iron Phosphate battery
pub const POWER_SUPPLY_TECHNOLOGY_NICD: u8 = 5; // Nickel-Cadmium battery
pub const POWER_SUPPLY_TECHNOLOGY_LIMN: u8 = 6; // Lithium Manganese Dioxide battery
pub const POWER_SUPPLY_TECHNOLOGY_TERNARY: u8 = 7; // Ternary Lithium battery
pub const POWER_SUPPLY_TECHNOLOGY_VRLA: u8 = 8; // Valve Regulated Lead-Acid battery

extern "C" {
    fn sensor_msgs__msg__BatteryState__init(msg: *mut BatteryState) -> bool;
    fn sensor_msgs__msg__BatteryState__fini(msg: *mut BatteryState);
    fn sensor_msgs__msg__BatteryState__are_equal(lhs: *const BatteryState, rhs: *const BatteryState) -> bool;
    fn sensor_msgs__msg__BatteryState__Sequence__init(msg: *mut BatteryStateSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__BatteryState__Sequence__fini(msg: *mut BatteryStateSeqRaw);
    fn sensor_msgs__msg__BatteryState__Sequence__are_equal(lhs: *const BatteryStateSeqRaw, rhs: *const BatteryStateSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__BatteryState() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct BatteryState {
    pub header: std_msgs::msg::Header,
    pub voltage: f32,
    pub temperature: f32,
    pub current: f32,
    pub charge: f32,
    pub capacity: f32,
    pub design_capacity: f32,
    pub percentage: f32,
    pub power_supply_status: u8,
    pub power_supply_health: u8,
    pub power_supply_technology: u8,
    pub present: bool,
    pub cell_voltage: crate::msg::F32Seq<0>,
    pub cell_temperature: crate::msg::F32Seq<0>,
    pub location: crate::msg::RosString<0>,
    pub serial_number: crate::msg::RosString<0>,
}

impl BatteryState {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__BatteryState__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for BatteryState {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__BatteryState__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct BatteryStateSeqRaw {
    data: *mut BatteryState,
    size: usize,
    capacity: usize,
}

/// Sequence of BatteryState.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct BatteryStateSeq<const N: usize> {
    data: *mut BatteryState,
    size: usize,
    capacity: usize,
}

impl<const N: usize> BatteryStateSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: BatteryStateSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__BatteryState__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: BatteryStateSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[BatteryState] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [BatteryState] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, BatteryState> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, BatteryState> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for BatteryStateSeq<N> {
    fn drop(&mut self) {
        let mut msg = BatteryStateSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__BatteryState__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for BatteryStateSeq<N> {}
unsafe impl<const N: usize> Sync for BatteryStateSeq<N> {}


impl TypeSupport for BatteryState {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__BatteryState()
        }
    }
}

impl PartialEq for BatteryState {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sensor_msgs__msg__BatteryState__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for BatteryStateSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = BatteryStateSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = BatteryStateSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            sensor_msgs__msg__BatteryState__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

