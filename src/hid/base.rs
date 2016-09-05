// exports from <IOKit/hid/IOHIDBase.h>

use cf::{CFDictionaryRef, CFIndex};
use libc::{c_void, uint32_t, uint8_t, uint64_t};
use ::{IOHIDReportType, IOReturn};

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDDevice {
    __private: c_void
}

pub type IOHIDDeviceRef = *mut __IOHIDDevice;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDElement {
    __private: c_void
}

pub type IOHIDElementRef = *mut __IOHIDElement;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDValue {
    __private: c_void
}

pub type IOHIDValueRef = *mut __IOHIDValue;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IOHIDTransactionDirectionType {
    kIOHIDTransactionDirectionTypeInput,
    kIOHIDTransactionDirectionTypeOutput
}

pub const kIOHIDTransactionOptionDefaultOutputValue: u32 = 0x0001;

pub type IOHIDCallback = unsafe extern "C" fn(context: *mut c_void, result: IOReturn, sender: *mut c_void);
pub type IOHIDReportCallback = unsafe extern "C" fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, 
                                                    _type: IOHIDReportType, reportID: uint32_t, report: *mut uint8_t, 
                                                    reportLength: CFIndex);
pub type IOHIDReportWithTimeStampCallback = unsafe extern "C" fn(context: *mut c_void, result: IOReturn, sender: *mut c_void, 
                                                                 _type: IOHIDReportType, reportID: uint32_t, report: *mut uint8_t, 
                                                                 reportLength: CFIndex, timestamp: uint64_t);
pub type IOHIDValueCallback = unsafe extern "C" fn(context: *mut c_void, result: IOReturn, sender: *mut c_void,
                                                   value: IOHIDValueRef);
pub type IOHIDValueMultipleCallback = unsafe extern "C" fn(context: *mut c_void, result: IOReturn, sender: *mut c_void,
                                                           multiple: CFDictionaryRef);
pub type IOHIDDeviceCallback = unsafe extern "C" fn(context: *mut c_void, result: IOReturn, sender: *mut c_void,
                                                    device: IOHIDDeviceRef);