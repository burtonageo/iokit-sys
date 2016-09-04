// exports from <IOKit/hid/IOHIDDevicePlugin.h>

use cf::{Boolean, CFArrayRef, CFDictionaryRef, CFIndex, CFStringRef, CFTypeRef,
         IUNKNOWN_C_GUTS};
use libc::{c_void, uint8_t, uint32_t};
use ::{IOHIDCallback, IOHIDElementRef, IOHIDReportType, IOHIDReportCallback, IOHIDReportWithTimeStampCallback,
       IOHIDTransactionDirectionType, IOHIDValueCallback, IOHIDValueRef, IOOptionBits, IOReturn};

macro_rules! kIOHIDDeviceFactoryID {
    () => {
        $crate::CFUUIDGetConstantUUIDWithBytes($crate::ptr::null_mut(),
            0x13, 0xAA, 0x9C, 0x44, 0x6F, 0x1B, 0x11, 0xD4,
            0x90, 0x7C, 0x00, 0x05, 0x02, 0x8F, 0x18, 0xD5)
    }
}

macro_rules! kIOHIDDeviceTypeID {
    () => {
        $crate::CFUUIDGetConstantUUIDWithBytes($crate::ptr::null_mut(), 
            0x7d, 0xde, 0xec, 0xa8, 0xa7, 0xb4, 0x11, 0xda,
            0x8a, 0x0e, 0x00, 0x14, 0x51, 0x97, 0x58, 0xef)
    }
}

macro_rules! kIOHIDDeviceDeviceInterfaceID {
    () => {
        $crate::CFUUIDGetConstantUUIDWithBytes($crate::ptr::null_mut(),
            0x47, 0x4b, 0xdc, 0x8e, 0x9f, 0x4a, 0x11, 0xda,
            0xb3, 0x66, 0x00, 0x0d, 0x93, 0x6d, 0x06, 0xd2)
    }
}

macro_rules! kIOHIDDeviceDeviceInterfaceID2 {
    () => {
        $crate::CFUUIDGetConstantUUIDWithBytes($crate::ptr::null_mut(),
            0xB4, 0x73, 0x25, 0x6C, 0x6A, 0x72, 0x4E, 0x04,
            0xB6, 0x94, 0xC4, 0x00, 0x1D, 0x20, 0x20, 0x20)
    }
}

macro_rules! kIOHIDDeviceQueueInterfaceID {
    () => {
        $crate::CFUUIDGetConstantUUIDWithBytes($crate::ptr::null_mut(),
            0x2e, 0xc7, 0x8b, 0xdb, 0x9f, 0x4e, 0x11, 0xda,
            0xb6, 0x5c, 0x00, 0x0d, 0x93, 0x6d, 0x06, 0xd2)
    }
}

macro_rules! kIOHIDDeviceTransactionInterfaceID {
    () => {
        $crate::CFUUIDGetConstantUUIDWithBytes($crate::ptr::null_mut(),
            0x1f, 0x2e, 0x78, 0xfa, 0x9f, 0xfa, 0x11, 0xda,
            0x90, 0xb4, 0x00, 0x0d, 0x93, 0x6d, 0x06, 0xd2)
    }
}

#[repr(C)]
pub struct IOHID_DEVICE_DEVICE_FUNCS_V1 {
    pub open: extern "C" fn(_self: *mut c_void, options: IOOptionBits) -> IOReturn,
    pub close: extern "C" fn(_self: *mut c_void, options: IOOptionBits) -> IOReturn,

    pub getProperty: extern "C" fn(_self: *mut c_void, key: CFStringRef, pProperty: *mut CFTypeRef) -> IOReturn,
    pub setProperty: extern "C" fn(_self: *mut c_void, key: CFStringRef, property: CFTypeRef)-> IOReturn,
    
    pub getAsyncEventSource: extern "C" fn(_self: *mut c_void, source: CFTypeRef) -> IOReturn,
    pub copyMatchingElements: extern "C" fn(_self: *mut c_void, matchingDict: CFDictionaryRef,
                                            pElements: *mut CFArrayRef, options: IOOptionBits)
                                            -> IOReturn,
    pub setValue: extern "C" fn(_self: *mut c_void, value: IOHIDValueRef, timeout: uint32_t,
                                                     callback: IOHIDValueCallback, context: *mut c_void,
                                                     options: IOOptionBits) -> IOReturn,
    pub getValue: extern "C" fn(_self: *mut c_void, pValue: *mut IOHIDValueRef, timeout: uint32_t,
                                                     callback: IOHIDValueCallback, context: *mut c_void,
                                                     options: IOOptionBits) -> IOReturn,
    
    pub setInputReportCallback: extern "C" fn(_self: *mut c_void, report: *mut uint8_t, reportID: uint32_t,
                                              reportLength: CFIndex, callback: IOHIDReportCallback,
                                              context: *mut c_void, options: IOOptionBits) -> IOReturn,
    
    pub setReport: extern "C" fn(_self: *mut c_void, reportType: IOHIDReportType, reportID: uint32_t, report: *const uint8_t,
                                 reportLength: CFIndex, timeout: uint32_t, callback: IOHIDReportCallback, context: *mut c_void,
                                 options: IOOptionBits) -> IOReturn,
    pub getReport: extern "C" fn(_self: *mut c_void, reportType: IOHIDReportType, reportID: uint32_t, report: *mut uint8_t,
                                 pReportLength: *mut CFIndex, timeout: uint32_t, callback: IOHIDReportCallback,
                                 context: *mut c_void, options: IOOptionBits) -> IOReturn,
}

#[repr(C)]
pub struct IOHIDDeviceDeviceInterface {
    pub iunknown_c_guts: IUNKNOWN_C_GUTS,
    pub device_funcs_v1: IOHID_DEVICE_DEVICE_FUNCS_V1
}

#[repr(C)]
pub struct IOHID_DEVICE_DEVICE_FUNCS_V2 {
    pub setInputReportWithTimeStampCallback: extern "C" fn(_self: *mut c_void, report: *mut uint8_t, reportLength: CFIndex,
                                                           callback: IOHIDReportWithTimeStampCallback, context: *mut c_void,
                                                           options: IOOptionBits) -> IOReturn
}

#[repr(C)]
pub struct IOHIDDeviceTimeStampedDeviceInterface {
    pub iunknown_c_guts: IUNKNOWN_C_GUTS,
    pub device_funcs_v1: IOHID_DEVICE_DEVICE_FUNCS_V1,
    pub device_funcs_v2: IOHID_DEVICE_DEVICE_FUNCS_V2
}

#[repr(C)]
pub struct IOHIDDeviceQueueInterface {
    pub iunknown_c_guts: IUNKNOWN_C_GUTS,

    pub getAsyncEventSource: extern "C" fn(_self: *mut c_void, pSource: *mut CFTypeRef) -> IOReturn,

    pub setDepth: extern "C" fn(_self: *mut c_void, depth: uint32_t, options: IOOptionBits) -> IOReturn,
    pub getDepth: extern "C" fn(_self: *mut c_void, pDepth: *mut uint32_t) -> IOReturn,

    pub addElement: extern "C" fn(_self: *mut c_void, element: IOHIDElementRef, options: IOOptionBits) -> IOReturn,
    pub removeElement: extern "C" fn(_self: *mut c_void, element: IOHIDElementRef, options: IOOptionBits) -> IOReturn,
    pub containsElement:  extern "C" fn(_self: *mut c_void, element: IOHIDElementRef, pValue: *mut Boolean,
                                        options: IOOptionBits) -> IOReturn,

    pub start: extern "C" fn(_self: *mut c_void, options: IOOptionBits) -> IOReturn,
    pub stop: extern "C" fn(_self: *mut c_void, options: IOOptionBits) -> IOReturn,

    pub setValueAvailableCallback: extern "C" fn(_self: *mut c_void, callback: IOHIDCallback, context: *mut c_void)
                                                 -> IOReturn,

    pub copyNextValue: extern "C" fn(_self: *mut c_void, pValue: *mut IOHIDValueRef, timeout: uint32_t, options: IOOptionBits)
                                     -> IOReturn
}

#[repr(C)]
pub struct IOHIDDeviceTransactionInterface {
    pub iunknown_c_guts: IUNKNOWN_C_GUTS,

    pub getAsyncEventSource: extern "C" fn(_self: *mut c_void, pSource: *mut CFTypeRef) -> IOReturn,

    pub setDirection: extern "C" fn(_self: *mut c_void, direction: IOHIDTransactionDirectionType, options: IOOptionBits)
                                    -> IOReturn,
    pub getDirection: extern "C" fn(_self: *mut c_void, pDirection: *mut IOHIDTransactionDirectionType) -> IOReturn,

    pub addElement: extern "C" fn(_self: *mut c_void, element: IOHIDElementRef, options: IOOptionBits) -> IOReturn,
    pub removeElement: extern "C" fn(_self: *mut c_void, element: IOHIDElementRef, options: IOOptionBits) -> IOReturn,
    pub containsElement: extern "C" fn(_self: *mut c_void, element: IOHIDElementRef, pValue: *mut Boolean,
                                       options: IOOptionBits) -> IOReturn,

    pub setValue: extern "C" fn(_self: *mut c_void, element: IOHIDElementRef, value: IOHIDValueRef, options: IOOptionBits)
                                -> IOReturn,
    pub getValue: extern "C" fn(_self: *mut c_void, element: IOHIDElementRef, pValue: *mut IOHIDValueRef, options: IOOptionBits)
                                -> IOReturn,

    pub commit: extern "C" fn(_self: *mut c_void, timeout: uint32_t, callback: IOHIDCallback, context: *mut c_void,
                              options: IOOptionBits) -> IOReturn,
    pub clear: extern "C" fn(_self: *mut c_void, options: IOOptionBits) -> IOReturn
}
