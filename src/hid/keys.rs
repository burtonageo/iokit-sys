// exports from <IOKit/hid/IOHIDKeys.h>

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
use libc::c_void; 
use libc::uint32_t;

pub const kIOHIDDeviceKey: &'static str = "IOHIDDevice\0";
pub const kIOHIDTransportKey: &'static str = "Transport\0";
pub const kIOHIDVendorIDKey: &'static str = "VendorID\0";
pub const kIOHIDVendorIDSourceKey: &'static str = "VendorIDSource\0";
pub const kIOHIDProductIDKey: &'static str = "ProductID\0";
pub const kIOHIDVersionNumberKey: &'static str = "VersionNumber\0";
pub const kIOHIDManufacturerKey: &'static str = "Manufacturer\0";
pub const kIOHIDProductKey: &'static str = "Product\0";
pub const kIOHIDSerialNumberKey: &'static str = "SerialNumber\0";
pub const kIOHIDCountryCodeKey: &'static str = "CountryCode\0";
pub const kIOHIDStandardTypeKey: &'static str = "StandardType\0";
pub const kIOHIDLocationIDKey: &'static str = "LocationID\0";
pub const kIOHIDDeviceUsageKey: &'static str = "DeviceUsage\0";
pub const kIOHIDDeviceUsagePageKey: &'static str = "DeviceUsagePage\0";
pub const kIOHIDDeviceUsagePairsKey: &'static str = "DeviceUsagePairs\0";
pub const kIOHIDPrimaryUsageKey: &'static str = "PrimaryUsage\0";
pub const kIOHIDPrimaryUsagePageKey: &'static str = "PrimaryUsagePage\0";
pub const kIOHIDMaxInputReportSizeKey: &'static str = "MaxInputReportSize\0";
pub const kIOHIDMaxOutputReportSizeKey: &'static str = "MaxOutputReportSize\0";
pub const kIOHIDMaxFeatureReportSizeKey: &'static str = "MaxFeatureReportSize\0";
pub const kIOHIDReportIntervalKey: &'static str = "ReportInterval\0";
pub const kIOHIDSampleIntervalKey: &'static str = "SampleInterval\0";
pub const kIOHIDBatchIntervalKey: &'static str = "BatchInterval\0";
pub const kIOHIDRequestTimeoutKey: &'static str = "RequestTimeout\0";
pub const kIOHIDReportDescriptorKey: &'static str = "ReportDescriptor\0";
pub const kIOHIDResetKey: &'static str = "Reset\0";
pub const kIOHIDKeyboardLanguageKey: &'static str = "KeyboardLanguage\0";
pub const kIOHIDAltHandlerIdKey: &'static str = "alt_handler_id\0";
pub const kIOHIDBuiltInKey: &'static str = "Built-In\0";
pub const kIOHIDDisplayIntegratedKey: &'static str = "DisplayIntegrated\0";
pub const kIOHIDProductIDMaskKey: &'static str = "ProductIDMask\0";
pub const kIOHIDProductIDArrayKey: &'static str = "ProductIDArray\0";
pub const kIOHIDPowerOnDelayNSKey: &'static str = "HIDPowerOnDelayNS\0";
pub const kIOHIDCategoryKey: &'static str = "Category\0";
pub const kIOHIDMaxResponseLatencyKey: &'static str = "MaxResponseLatency\0";
pub const kIOHIDUniqueIDKey: &'static str = "UniqueID\0";
pub const kIOHIDPhysicalDeviceUniqueIDKey: &'static str = "PhysicalDeviceUniqueID\0";

pub const kIOHIDTransportUSBValue: &'static str = "USB\0";
pub const kIOHIDTransportBluetoothValue: &'static str = "Bluetooth\0";
pub const kIOHIDTransportBluetoothLowEnergyValue: &'static str = "BluetoothLowEnergy\0";
pub const kIOHIDTransportAIDBValue: &'static str = "AIDB\0";
pub const kIOHIDTransportI2CValue: &'static str = "I2C\0";
pub const kIOHIDTransportSPIValue: &'static str = "SPI\0";
pub const kIOHIDTransportSerialValue: &'static str = "Serial\0";
pub const kIOHIDTransportIAPValue: &'static str = "IAP\0";
pub const kIOHIDTransportAirPlayValue: &'static str = "AirPlay\0";
pub const kIOHIDTransportSPUValue: &'static str = "SPU\0";

pub const kIOHIDCategoryAutomotiveValue: &'static str = "Automotive\0";

pub const kIOHIDElementKey: &'static str = "Elements\0";

pub const kIOHIDElementCookieKey: &'static str = "ElementCookie\0";
pub const kIOHIDElementTypeKey: &'static str = "Type\0";
pub const kIOHIDElementCollectionTypeKey: &'static str = "CollectionType\0";
pub const kIOHIDElementUsageKey: &'static str = "Usage\0";
pub const kIOHIDElementUsagePageKey: &'static str = "UsagePage\0";
pub const kIOHIDElementMinKey: &'static str = "Min\0";
pub const kIOHIDElementMaxKey: &'static str = "Max\0";
pub const kIOHIDElementScaledMinKey: &'static str = "ScaledMin\0";
pub const kIOHIDElementScaledMaxKey: &'static str = "ScaledMax\0";
pub const kIOHIDElementSizeKey: &'static str = "Size\0";
pub const kIOHIDElementReportSizeKey: &'static str = "ReportSize\0";
pub const kIOHIDElementReportCountKey: &'static str = "ReportCount\0";
pub const kIOHIDElementReportIDKey: &'static str = "ReportID\0";
pub const kIOHIDElementIsArrayKey: &'static str = "IsArray\0";
pub const kIOHIDElementIsRelativeKey: &'static str = "IsRelative\0";
pub const kIOHIDElementIsWrappingKey: &'static str = "IsWrapping\0";
pub const kIOHIDElementIsNonLinearKey: &'static str = "IsNonLinear\0";
pub const kIOHIDElementHasPreferredStateKey: &'static str = "HasPreferredState\0";
pub const kIOHIDElementHasNullStateKey: &'static str = "HasNullState\0";
pub const kIOHIDElementFlagsKey: &'static str = "Flags\0";
pub const kIOHIDElementUnitKey: &'static str = "Unit\0";
pub const kIOHIDElementUnitExponentKey: &'static str = "UnitExponent\0";
pub const kIOHIDElementNameKey: &'static str = "Name\0";
pub const kIOHIDElementValueLocationKey: &'static str = "ValueLocation\0";
pub const kIOHIDElementDuplicateIndexKey: &'static str = "DuplicateIndex\0";
pub const kIOHIDElementParentCollectionKey: &'static str = "ParentCollection\0";

#[cfg(not(target_arch = "powerpc"))]
pub const kIOHIDElementVendorSpecificKey: &'static str = "VendorSpecific\0";

#[cfg(target_arch = "powerpc")]
pub const kIOHIDElementVendorSpecificKey:&'static str = "VendorSpecifc\0";

pub const kIOHIDElementCookieMinKey: &'static str = "ElementCookieMin\0";
pub const kIOHIDElementCookieMaxKey: &'static str = "ElementCookieMax\0";
pub const kIOHIDElementUsageMinKey: &'static str = "UsageMin\0";
pub const kIOHIDElementUsageMaxKey: &'static str = "UsageMax\0";

pub const kIOHIDElementCalibrationMinKey: &'static str = "CalibrationMin\0";
pub const kIOHIDElementCalibrationMaxKey: &'static str = "CalibrationMax\0";
pub const kIOHIDElementCalibrationSaturationMinKey: &'static str = "CalibrationSaturationMin\0";
pub const kIOHIDElementCalibrationSaturationMaxKey: &'static str = "CalibrationSaturationMax\0";
pub const kIOHIDElementCalibrationDeadZoneMinKey: &'static str = "CalibrationDeadZoneMin\0";
pub const kIOHIDElementCalibrationDeadZoneMaxKey: &'static str = "CalibrationDeadZoneMax\0";
pub const kIOHIDElementCalibrationGranularityKey: &'static str = "CalibrationGranularity\0";

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
