// exports from <IOKit/hid/IOHIDValue.h>

use cf::{CFAllocatorRef, CFIndex, CFTypeID};
use libc::{uint64_t, uint8_t};
use ::{IOHIDElementRef, IOHIDValueRef, IOHIDValueScaleType};

extern "C" {
    pub fn IOHIDValueGetTypeID() -> CFTypeID;

    pub fn IOHIDValueCreateWithIntegerValue(allocator: CFAllocatorRef, timestamp: uint64_t, value: CFIndex) -> IOHIDValueRef;
    pub fn IOHIDValueCreateWithBytes(allocator: CFAllocatorRef, timestamp: uint64_t, bytes: *const uint8_t, length: CFIndex)
                                     -> IOHIDValueRef;
    pub fn IOHIDValueCreateWithBytesNoCopy(allocator: CFAllocatorRef, timestamp: uint64_t, bytes: *const uint8_t,
                                           length: CFIndex) -> IOHIDValueRef;

    pub fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;
    pub fn IOHIDValueGetTimeStamp(value: IOHIDValueRef) -> uint64_t;
    pub fn IOHIDValueGetLength(value: IOHIDValueRef) -> CFIndex;
    pub fn IOHIDValueGetBytePtr(value: IOHIDValueRef) -> *const uint8_t;
    pub fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> CFIndex;
    pub fn IOHIDValueGetScaledValue(value: IOHIDValueRef, _type: IOHIDValueScaleType) -> f64; // TODO: return double_t type
}
