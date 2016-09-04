// exports from <IOKit/hid/IOHIDTransaction.h>

use cf::{Boolean, CFAllocatorRef, CFRunLoopRef, CFStringRef, CFTimeInterval, CFTypeID};
use libc::c_void;
use ::{IOHIDDeviceRef, IOHIDCallback, IOHIDElementRef, IOHIDTransactionDirectionType, IOHIDValueRef, IOReturn, IOOptionBits};

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDTransaction {
    __private: c_void
}

pub type IOHIDTransactionRef = *mut __IOHIDTransaction;

extern "C" {
    pub fn IOHIDTransactionGetTypeID() -> CFTypeID;

    pub fn IOHIDTransactionCreate(allocator: CFAllocatorRef, device: IOHIDDeviceRef, direction: IOHIDTransactionDirectionType,
                                  options: IOOptionBits) -> IOHIDTransactionRef;

    pub fn IOHIDTransactionGetDevice(transaction: IOHIDTransactionRef) -> IOHIDDeviceRef;
    pub fn IOHIDTransactionGetDirection(transaction: IOHIDTransactionRef) -> IOHIDTransactionDirectionType;
    pub fn IOHIDTransactionSetDirection(transaction: IOHIDTransactionRef, direction: IOHIDTransactionDirectionType);

    pub fn IOHIDTransactionAddElement(transaction: IOHIDTransactionRef, element: IOHIDElementRef);
    pub fn IOHIDTransactionRemoveElement(transaction: IOHIDTransactionRef, element: IOHIDElementRef);
    pub fn IOHIDTransactionContainsElement(transaction: IOHIDTransactionRef, element: IOHIDElementRef) -> Boolean;

    pub fn IOHIDTransactionScheduleWithRunLoop(transaction: IOHIDTransactionRef, runLoop: CFRunLoopRef,
                                               runLoopMode: CFStringRef);
    pub fn IOHIDTransactionUnscheduleFromRunLoop(transaction: IOHIDTransactionRef, runLoop: CFRunLoopRef,
                                                 runLoopMode: CFStringRef);

    pub fn IOHIDTransactionSetValue(transaction: IOHIDTransactionRef, element: IOHIDElementRef, value: IOHIDValueRef,
                                    options: IOOptionBits);
    pub fn IOHIDTransactionGetValue(transaction: IOHIDTransactionRef, element: IOHIDElementRef, options: IOOptionBits)
                                    -> IOHIDValueRef;

    pub fn IOHIDTransactionCommit(transaction: IOHIDTransactionRef) -> IOReturn;
    pub fn IOHIDTransactionCommitWithCallback(transaction: IOHIDTransactionRef, timeout: CFTimeInterval,
                                              callback: IOHIDCallback, context: *mut c_void) -> IOReturn;
    pub fn IOHIDTransactionClear(transaction: IOHIDTransactionRef);
}
