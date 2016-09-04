// exports from <IOKit/hid/IOHIDElement.h>

use cf::{Boolean, CFAllocatorRef, CFArrayRef, CFDictionaryRef, CFIndex, CFStringRef, CFTypeID, CFTypeRef};
use libc::uint32_t;
use ::{IOHIDDeviceRef, IOHIDElementCookie, IOHIDElementRef, IOHIDElementType, IOHIDElementCollectionType};

extern "C" {
    pub fn IOHIDElementGetTypeID() -> CFTypeID;

    pub fn IOHIDElementCreateWithDictionary(allocator: CFAllocatorRef, dictionary: CFDictionaryRef) -> IOHIDElementRef;

    pub fn IOHIDElementGetDevice(element: IOHIDElementRef) -> IOHIDDeviceRef;
    pub fn IOHIDElementGetParent(element: IOHIDElementRef) -> IOHIDElementRef;
    pub fn IOHIDElementGetChildren(element: IOHIDElementRef) -> CFArrayRef;

    pub fn IOHIDElementAttach(element: IOHIDElementRef, toAttach: IOHIDElementRef);
    pub fn IOHIDElementDetach(element: IOHIDElementRef, toDetach: IOHIDElementRef);
    pub fn IOHIDElementCopyAttached(element: IOHIDElementRef) -> CFArrayRef;

    pub fn IOHIDElementGetCookie(element: IOHIDElementRef) -> IOHIDElementCookie;
    pub fn IOHIDElementGetType(element: IOHIDElementRef) -> IOHIDElementType;
    pub fn IOHIDElementGetCollectionType(element: IOHIDElementRef) -> IOHIDElementCollectionType;
    pub fn IOHIDElementGetUsagePage(element: IOHIDElementRef) -> uint32_t;
    pub fn IOHIDElementGetUsage(element: IOHIDElementRef) -> uint32_t;

    pub fn IOHIDElementIsVirtual(element: IOHIDElementRef) -> Boolean;
    pub fn IOHIDElementIsRelative(element: IOHIDElementRef) -> Boolean;
    pub fn IOHIDElementIsWrapping(element: IOHIDElementRef) -> Boolean;
    pub fn IOHIDElementIsArray(element: IOHIDElementRef) -> Boolean;
    pub fn IOHIDElementIsNonLinear(element: IOHIDElementRef) -> Boolean;
    pub fn IOHIDElementHasPreferredState(element: IOHIDElementRef) -> Boolean;
    pub fn IOHIDElementHasNullState(element: IOHIDElementRef) -> Boolean;

    pub fn IOHIDElementGetName(element: IOHIDElementRef) -> CFStringRef;
    pub fn IOHIDElementGetReportID(element: IOHIDElementRef) -> uint32_t;
    pub fn IOHIDElementGetReportSize(element: IOHIDElementRef) -> uint32_t;
    pub fn IOHIDElementGetReportCount(element: IOHIDElementRef) -> uint32_t;
    pub fn IOHIDElementGetUnit(element: IOHIDElementRef) -> uint32_t;
    pub fn IOHIDElementGetUnitExponent(element: IOHIDElementRef) -> uint32_t;

    pub fn IOHIDElementGetLogicalMin(element: IOHIDElementRef) -> CFIndex;
    pub fn IOHIDElementGetLogicalMax(element: IOHIDElementRef) -> CFIndex;
    pub fn IOHIDElementGetPhysicalMin(element: IOHIDElementRef) -> CFIndex;
    pub fn IOHIDElementGetPhysicalMax(element: IOHIDElementRef) -> CFIndex;

    pub fn IOHIDElementGetProperty(element: IOHIDElementRef, key: CFStringRef) -> CFTypeRef;
    pub fn IOHIDElementSetProperty(element: IOHIDElementRef, key: CFStringRef, property: CFTypeRef) -> Boolean;
}
