// exports from <IOKit/hid/IOHIDQueue.h>

use cf::{Boolean, CFAllocatorRef, CFIndex, CFRunLoopRef, CFStringRef, CFTimeInterval, CFTypeID};
use libc::c_void;
use ::{IOHIDCallback, IOHIDDeviceRef, IOHIDElementRef, IOOptionBits};

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDQueue {
    __private: c_void
}

pub type IOHIDQueueRef = *mut __IOHIDQueue;

extern "C" {
    pub fn IOHIDQueueGetTypeID() -> CFTypeID;

    pub fn IOHIDQueueCreate(allocator: CFAllocatorRef, device: IOHIDDeviceRef, depth: CFIndex, options: IOOptionBits)
                            -> IOHIDQueueRef;

    pub fn IOHIDQueueGetDevice(queue: IOHIDQueueRef) -> IOHIDDeviceRef;
    pub fn IOHIDQueueGetDepth(queue: IOHIDQueueRef) -> CFIndex;
    pub fn IOHIDQueueSetDepth(queue: IOHIDQueueRef, depth: CFIndex);

    pub fn IOHIDQueueAddElement(queue: IOHIDQueueRef, element: IOHIDElementRef);
    pub fn IOHIDQueueRemoveElement(queue: IOHIDQueueRef, element: IOHIDElementRef);
    pub fn IOHIDQueueContainsElement(queue: IOHIDQueueRef, element: IOHIDElementRef) -> Boolean;

    pub fn IOHIDQueueStart(queue: IOHIDQueueRef);
    pub fn IOHIDQueueStop(queue: IOHIDQueueRef);
    pub fn IOHIDQueueScheduleWithRunLoop(queue: IOHIDQueueRef, runLoop: CFRunLoopRef, runLoopMode: CFStringRef);
    pub fn IOHIDQueueUnscheduleFromRunLoop(queue: IOHIDQueueRef, runLoop: CFRunLoopRef, runLoopMode: CFStringRef);
    
    pub fn IOHIDQueueRegisterValueAvailableCallback(queue: IOHIDQueueRef, callback: IOHIDCallback, context: *mut c_void);

    pub fn IOHIDQueueCopyNextValue(queue: IOHIDQueueRef) -> IOHIDElementRef;
    pub fn IOHIDQueueCopyNextValueWithTimeout(queue: IOHIDQueueRef, timeout: CFTimeInterval) -> IOHIDElementRef;
}
