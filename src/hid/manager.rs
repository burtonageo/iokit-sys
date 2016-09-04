// exports from <IOKit/hid/IOHIDManager.h>

use cf::{Boolean, CFAllocatorRef, CFArrayRef, CFDictionaryRef, CFRunLoopRef, CFSetRef, CFStringRef, CFTypeID, CFTypeRef};
use libc::c_void;
use ::{IOHIDDeviceCallback, IOHIDValueCallback, IOOptionBits, IOReturn};

pub const kIOHIDManagerOptionNone: IOOptionBits = 0x0;
pub const kIOHIDManagerOptionUsePersistentProperties: IOOptionBits = 0x1;
pub const kIOHIDManagerOptionDoNotLoadProperties: IOOptionBits = 0x2;
pub const kIOHIDManagerOptionDoNotSaveProperties: IOOptionBits = 0x4;

#[doc(hidden)]
#[repr(C)]
pub struct __IOHIDDeviceManager {
    __private: c_void
}

pub type IOHIDDeviceManagerRef = *mut __IOHIDDeviceManager;

extern {
    pub fn IOHIDManagerGetTypeID() -> CFTypeID;

    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOOptionBits) -> IOHIDDeviceManagerRef;
    pub fn IOHIDManagerOpen(manager: IOHIDDeviceManagerRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDManagerClose(manager: IOHIDDeviceManagerRef, options: IOOptionBits) -> IOReturn;

    pub fn IOHIDManagerGetProperty(manager: IOHIDDeviceManagerRef, key: CFStringRef) -> CFTypeRef;
    pub fn IOHIDManagerSetProperty(manager: IOHIDDeviceManagerRef, key: CFStringRef, value: CFTypeRef)
                                   -> Boolean;

    pub fn IOHIDManagerScheduleWithRunLoop(manager: IOHIDDeviceManagerRef, runLoop: CFRunLoopRef,
                                           runLoopMode: CFStringRef);
    pub fn IOHIDManagerUnscheduleFromRunLoop(manager: IOHIDDeviceManagerRef, runLoop: CFRunLoopRef,
                                             runLoopMode: CFStringRef);

    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDDeviceManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerSetDeviceMatchingMultiple(manager: IOHIDDeviceManagerRef, multiple: CFArrayRef);

    pub fn IOHIDManagerCopyDevices(manager: IOHIDDeviceManagerRef) -> CFSetRef;

    pub fn IOHIDManagerRegisterDeviceMatchingCallback(manager: IOHIDDeviceManagerRef, callback: IOHIDDeviceCallback,
                                                      context: *mut c_void);
    pub fn IOHIDManagerRegisterDeviceRemovalCallback(manager: IOHIDDeviceManagerRef, callback: IOHIDDeviceCallback,
                                                     context: *mut c_void);
    pub fn IOHIDManagerRegisterInputReportCallback(manager: IOHIDDeviceManagerRef, callback: IOHIDDeviceCallback,
                                                   context: *mut c_void);
    pub fn IOHIDManagerRegisterInputValueCallback(manager: IOHIDDeviceManagerRef, callback: IOHIDValueCallback,
                                                  context: *mut c_void);

    pub fn IOHIDManagerSetInputValueMatching(manager: IOHIDDeviceManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerSetInputValueMatchingMultiple(manager: IOHIDDeviceManagerRef, multiple: CFArrayRef);

    pub fn IOHIDManagerSaveToPropertyDomain(manager: IOHIDDeviceManagerRef, applicationID: CFStringRef,
                                            userName: CFStringRef, hostName: CFStringRef, options: IOOptionBits);
}
