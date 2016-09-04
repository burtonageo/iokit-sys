// exports from <IOKit/hid/IOHIDKeys.h>

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
use libc::c_void; 
use libc::uint32_t;

pub const kIOHIDDeviceKey: &'static [u8] = b"IOHIDDevice\0";
pub const kIOHIDTransportKey: &'static [u8] = b"Transport\0";
pub const kIOHIDVendorIDKey: &'static [u8] = b"VendorID\0";
pub const kIOHIDVendorIDSourceKey: &'static [u8] = b"VendorIDSource\0";
pub const kIOHIDProductIDKey: &'static [u8] = b"ProductID\0";
pub const kIOHIDVersionNumberKey: &'static [u8] = b"VersionNumber\0";
pub const kIOHIDManufacturerKey: &'static [u8] = b"Manufacturer\0";
pub const kIOHIDProductKey: &'static [u8] = b"Product\0";
pub const kIOHIDSerialNumberKey: &'static [u8] = b"SerialNumber\0";
pub const kIOHIDCountryCodeKey: &'static [u8] = b"CountryCode\0";
pub const kIOHIDStandardTypeKey: &'static [u8] = b"StandardType\0";
pub const kIOHIDLocationIDKey: &'static [u8] = b"LocationID\0";
pub const kIOHIDDeviceUsageKey: &'static [u8] = b"DeviceUsage\0";
pub const kIOHIDDeviceUsagePageKey: &'static [u8] = b"DeviceUsagePage\0";
pub const kIOHIDDeviceUsagePairsKey: &'static [u8] = b"DeviceUsagePairs\0";
pub const kIOHIDPrimaryUsageKey: &'static [u8] = b"PrimaryUsage\0";
pub const kIOHIDPrimaryUsagePageKey: &'static [u8] = b"PrimaryUsagePage\0";
pub const kIOHIDMaxInputReportSizeKey: &'static [u8] = b"MaxInputReportSize\0";
pub const kIOHIDMaxOutputReportSizeKey: &'static [u8] = b"MaxOutputReportSize\0";
pub const kIOHIDMaxFeatureReportSizeKey: &'static [u8] = b"MaxFeatureReportSize\0";
pub const kIOHIDReportIntervalKey: &'static [u8] = b"ReportInterval\0";
pub const kIOHIDSampleIntervalKey: &'static [u8] = b"SampleInterval\0";
pub const kIOHIDBatchIntervalKey: &'static [u8] = b"BatchInterval\0";
pub const kIOHIDRequestTimeoutKey: &'static [u8] = b"RequestTimeout\0";
pub const kIOHIDReportDescriptorKey: &'static [u8] = b"ReportDescriptor\0";
pub const kIOHIDResetKey: &'static [u8] = b"Reset\0";
pub const kIOHIDKeyboardLanguageKey: &'static [u8] = b"KeyboardLanguage\0";
pub const kIOHIDAltHandlerIdKey: &'static [u8] = b"alt_handler_id\0";
pub const kIOHIDBuiltInKey: &'static [u8] = b"Built-In\0";
pub const kIOHIDDisplayIntegratedKey: &'static [u8] = b"DisplayIntegrated\0";
pub const kIOHIDProductIDMaskKey: &'static [u8] = b"ProductIDMask\0";
pub const kIOHIDProductIDArrayKey: &'static [u8] = b"ProductIDArray\0";
pub const kIOHIDPowerOnDelayNSKey: &'static [u8] = b"HIDPowerOnDelayNS\0";
pub const kIOHIDCategoryKey: &'static [u8] = b"Category\0";
pub const kIOHIDMaxResponseLatencyKey: &'static [u8] = b"MaxResponseLatency\0";
pub const kIOHIDUniqueIDKey: &'static [u8] = b"UniqueID\0";
pub const kIOHIDPhysicalDeviceUniqueIDKey: &'static [u8] = b"PhysicalDeviceUniqueID\0";

pub const kIOHIDTransportUSBValue: &'static [u8] = b"USB\0";
pub const kIOHIDTransportBluetoothValue: &'static [u8] = b"Bluetooth\0";
pub const kIOHIDTransportBluetoothLowEnergyValue: &'static [u8] = b"BluetoothLowEnergy\0";
pub const kIOHIDTransportAIDBValue: &'static [u8] = b"AIDB\0";
pub const kIOHIDTransportI2CValue: &'static [u8] = b"I2C\0";
pub const kIOHIDTransportSPIValue: &'static [u8] = b"SPI\0";
pub const kIOHIDTransportSerialValue: &'static [u8] = b"Serial\0";
pub const kIOHIDTransportIAPValue: &'static [u8] = b"IAP\0";
pub const kIOHIDTransportAirPlayValue: &'static [u8] = b"AirPlay\0";
pub const kIOHIDTransportSPUValue: &'static [u8] = b"SPU\0";

pub const kIOHIDCategoryAutomotiveValue: &'static [u8] = b"Automotive\0";

pub const kIOHIDElementKey: &'static [u8] = b"Elements\0";

pub const kIOHIDElementCookieKey: &'static [u8] = b"ElementCookie\0";
pub const kIOHIDElementTypeKey: &'static [u8] = b"Type\0";
pub const kIOHIDElementCollectionTypeKey: &'static [u8] = b"CollectionType\0";
pub const kIOHIDElementUsageKey: &'static [u8] = b"Usage\0";
pub const kIOHIDElementUsagePageKey: &'static [u8] = b"UsagePage\0";
pub const kIOHIDElementMinKey: &'static [u8] = b"Min\0";
pub const kIOHIDElementMaxKey: &'static [u8] = b"Max\0";
pub const kIOHIDElementScaledMinKey: &'static [u8] = b"ScaledMin\0";
pub const kIOHIDElementScaledMaxKey: &'static [u8] = b"ScaledMax\0";
pub const kIOHIDElementSizeKey: &'static [u8] = b"Size\0";
pub const kIOHIDElementReportSizeKey: &'static [u8] = b"ReportSize\0";
pub const kIOHIDElementReportCountKey: &'static [u8] = b"ReportCount\0";
pub const kIOHIDElementReportIDKey: &'static [u8] = b"ReportID\0";
pub const kIOHIDElementIsArrayKey: &'static [u8] = b"IsArray\0";
pub const kIOHIDElementIsRelativeKey: &'static [u8] = b"IsRelative\0";
pub const kIOHIDElementIsWrappingKey: &'static [u8] = b"IsWrapping\0";
pub const kIOHIDElementIsNonLinearKey: &'static [u8] = b"IsNonLinear\0";
pub const kIOHIDElementHasPreferredStateKey: &'static [u8] = b"HasPreferredState\0";
pub const kIOHIDElementHasNullStateKey: &'static [u8] = b"HasNullState\0";
pub const kIOHIDElementFlagsKey: &'static [u8] = b"Flags\0";
pub const kIOHIDElementUnitKey: &'static [u8] = b"Unit\0";
pub const kIOHIDElementUnitExponentKey: &'static [u8] = b"UnitExponent\0";
pub const kIOHIDElementNameKey: &'static [u8] = b"Name\0";
pub const kIOHIDElementValueLocationKey: &'static [u8] = b"ValueLocation\0";
pub const kIOHIDElementDuplicateIndexKey: &'static [u8] = b"DuplicateIndex\0";
pub const kIOHIDElementParentCollectionKey: &'static [u8] = b"ParentCollection\0";

#[cfg(not(target_arch = "powerpc"))]
pub const kIOHIDElementVendorSpecificKey: &'static [u8] = b"VendorSpecific\0";

#[cfg(target_arch = "powerpc")]
pub const kIOHIDElementVendorSpecificKey:&'static [u8] = b"VendorSpecifc\0";

pub const kIOHIDElementCookieMinKey: &'static [u8] = b"ElementCookieMin\0";
pub const kIOHIDElementCookieMaxKey: &'static [u8] = b"ElementCookieMax\0";
pub const kIOHIDElementUsageMinKey: &'static [u8] = b"UsageMin\0";
pub const kIOHIDElementUsageMaxKey: &'static [u8] = b"UsageMax\0";

pub const kIOHIDElementCalibrationMinKey: &'static [u8] = b"CalibrationMin\0";
pub const kIOHIDElementCalibrationMaxKey: &'static [u8] = b"CalibrationMax\0";
pub const kIOHIDElementCalibrationSaturationMinKey: &'static [u8] = b"CalibrationSaturationMin\0";
pub const kIOHIDElementCalibrationSaturationMaxKey: &'static [u8] = b"CalibrationSaturationMax\0";
pub const kIOHIDElementCalibrationDeadZoneMinKey: &'static [u8] = b"CalibrationDeadZoneMin\0";
pub const kIOHIDElementCalibrationDeadZoneMaxKey: &'static [u8] = b"CalibrationDeadZoneMax\0";
pub const kIOHIDElementCalibrationGranularityKey: &'static [u8] = b"CalibrationGranularity\0";

#[cfg(all(target_os = "windows", target_pointer_width = "64"))] // LP64
pub type IOHIDElementCookie = uint32_t;

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
pub type IOHIDElementCookie = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IOHIDElementType {
    kIOHIDElementTypeInput_Misc        = 1,
    kIOHIDElementTypeInput_Button      = 2,
    kIOHIDElementTypeInput_Axis        = 3,
    kIOHIDElementTypeInput_ScanCodes   = 4,
    kIOHIDElementTypeOutput            = 129,
    kIOHIDElementTypeFeature           = 257,
    kIOHIDElementTypeCollection        = 513
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IOHIDElementCollectionType {
    kIOHIDElementCollectionTypePhysical    = 0x00,
    kIOHIDElementCollectionTypeApplication,
    kIOHIDElementCollectionTypeLogical,
    kIOHIDElementCollectionTypeReport,
    kIOHIDElementCollectionTypeNamedArray,
    kIOHIDElementCollectionTypeUsageSwitch,
    kIOHIDElementCollectionTypeUsageModifier
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IOHIDReportType {
    kIOHIDReportTypeInput = 0,
    kIOHIDReportTypeOutput,
    kIOHIDReportTypeFeature,
    kIOHIDReportTypeCount
}

pub type IOHIDOptionsType = uint32_t;

pub const kIOHIDOptionsTypeNone: IOHIDOptionsType = 0x00;
pub const kIOHIDOptionsTypeSeizeDevice: IOHIDOptionsType = 0x01;

pub type IOHIDQueueOptionsType = uint32_t;

pub const kIOHIDQueueOptionsTypeNone: IOHIDQueueOptionsType = 0x00;
pub const kIOHIDQueueOptionsTypeEnqueueAll: IOHIDQueueOptionsType = 0x01;

pub type IOHIDElementFlags = uint32_t;

pub const kIOHIDElementFlagsConstantMask: IOHIDElementFlags = 0x0001;
pub const kIOHIDElementFlagsVariableMask: IOHIDElementFlags = 0x0002;
pub const kIOHIDElementFlagsRelativeMask: IOHIDElementFlags = 0x0004;
pub const kIOHIDElementFlagsWrapMask: IOHIDElementFlags = 0x0008;
pub const kIOHIDElementFlagsNonLinearMask: IOHIDElementFlags = 0x0010;
pub const kIOHIDElementFlagsNoPreferredMask: IOHIDElementFlags = 0x0020;
pub const kIOHIDElementFlagsNullStateMask: IOHIDElementFlags = 0x0040;
pub const kIOHIDElementFlagsVolativeMask: IOHIDElementFlags = 0x0080;
pub const kIOHIDElementFlagsBufferedByteMask: IOHIDElementFlags = 0x0100;

pub type IOHIDStandardType = uint32_t;

pub const kIOHIDStandardTypeANSI: IOHIDStandardType = 0;
pub const kIOHIDStandardTypeISO: IOHIDStandardType = 1;
pub const kIOHIDStandardTypeJIS: IOHIDStandardType = 2;

pub type IOHIDValueScaleType = uint32_t;

pub const kIOHIDValueOptionsFlagRelativeSimple: IOHIDValueScaleType = (1<<0);
pub const kIOHIDValueOptionsFlagPrevious: IOHIDValueScaleType = (1<<1);
