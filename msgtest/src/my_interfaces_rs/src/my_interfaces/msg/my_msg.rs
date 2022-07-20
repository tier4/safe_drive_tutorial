// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use safe_drive::msg::*;
use safe_drive::rcl;
use safe_drive::msg::common_interfaces::*;

extern "C" {
    fn my_interfaces__msg__MyMsg__init(msg: *mut MyMsg) -> bool;
    fn my_interfaces__msg__MyMsg__fini(msg: *mut MyMsg);
    fn my_interfaces__msg__MyMsg__Sequence__init(msg: *mut MyMsgSeqRaw, size: usize) -> bool;
    fn my_interfaces__msg__MyMsg__Sequence__fini(msg: *mut MyMsgSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__my_interfaces__msg__MyMsg() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct MyMsg {
    pub integer_value: i32,
    pub unbounded_integer_array: safe_drive::msg::I32Seq<0>,
    pub five_integers_array: [i32; 5],
    pub up_to_five_integers_array: safe_drive::msg::I32Seq<5>,
}

impl MyMsg {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { my_interfaces__msg__MyMsg__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyMsg {
    fn drop(&mut self) {
        unsafe { my_interfaces__msg__MyMsg__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyMsgSeqRaw {
    data: *mut MyMsg,
    size: usize,
    capacity: usize,
}

/// Sequence of MyMsg.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyMsgSeq<const N: usize> {
    data: *mut MyMsg,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyMsgSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: MyMsgSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { my_interfaces__msg__MyMsg__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[MyMsg]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [MyMsg]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for MyMsgSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyMsgSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { my_interfaces__msg__MyMsg__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyMsgSeq<N> {}
unsafe impl<const N: usize> Sync for MyMsgSeq<N> {}


impl TopicMsg for MyMsg {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__my_interfaces__msg__MyMsg()
        }
    }
}
