// exports from <IOKit/hid/IOHIDKeys.h>

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
use libc::c_void; 
use libc::uint32_t;

pub const kIOHIDDeviceKey: &'static str = "IOHIDDevice";
pub const kIOHIDTransportKey: &'static str = "Transport";
pub const kIOHIDVendorIDKey: &'static str = "VendorID";
pub const kIOHIDVendorIDSourceKey: &'static str = "VendorIDSource";
pub const kIOHIDProductIDKey: &'static str = "ProductID";
pub const kIOHIDVersionNumberKey: &'static str = "VersionNumber";
pub const kIOHIDManufacturerKey: &'static str = "Manufacturer";
pub const kIOHIDProductKey: &'static str = "Product";
pub const kIOHIDSerialNumberKey: &'static str = "SerialNumber";
pub const kIOHIDCountryCodeKey: &'static str = "CountryCode";
pub const kIOHIDStandardTypeKey: &'static str = "StandardType";
pub const kIOHIDLocationIDKey: &'static str = "LocationID";
pub const kIOHIDDeviceUsageKey: &'static str = "DeviceUsage";
pub const kIOHIDDeviceUsagePageKey: &'static str = "DeviceUsagePage";
pub const kIOHIDDeviceUsagePairsKey: &'static str = "DeviceUsagePairs";
pub const kIOHIDPrimaryUsageKey: &'static str = "PrimaryUsage";
pub const kIOHIDPrimaryUsagePageKey: &'static str = "PrimaryUsagePage";
pub const kIOHIDMaxInputReportSizeKey: &'static str = "MaxInputReportSize";
pub const kIOHIDMaxOutputReportSizeKey: &'static str = "MaxOutputReportSize";
pub const kIOHIDMaxFeatureReportSizeKey: &'static str = "MaxFeatureReportSize";
pub const kIOHIDReportIntervalKey: &'static str = "ReportInterval";
pub const kIOHIDSampleIntervalKey: &'static str = "SampleInterval";
pub const kIOHIDBatchIntervalKey: &'static str = "BatchInterval";
pub const kIOHIDRequestTimeoutKey: &'static str = "RequestTimeout";
pub const kIOHIDReportDescriptorKey: &'static str = "ReportDescriptor";
pub const kIOHIDResetKey: &'static str = "Reset";
pub const kIOHIDKeyboardLanguageKey: &'static str = "KeyboardLanguage";
pub const kIOHIDAltHandlerIdKey: &'static str = "alt_handler_id";
pub const kIOHIDBuiltInKey: &'static str = "Built-In";
pub const kIOHIDDisplayIntegratedKey: &'static str = "DisplayIntegrated";
pub const kIOHIDProductIDMaskKey: &'static str = "ProductIDMask";
pub const kIOHIDProductIDArrayKey: &'static str = "ProductIDArray";
pub const kIOHIDPowerOnDelayNSKey: &'static str = "HIDPowerOnDelayNS";
pub const kIOHIDCategoryKey: &'static str = "Category";
pub const kIOHIDMaxResponseLatencyKey: &'static str = "MaxResponseLatency";
pub const kIOHIDUniqueIDKey: &'static str = "UniqueID";
pub const kIOHIDPhysicalDeviceUniqueIDKey: &'static str = "PhysicalDeviceUniqueID";

pub const kIOHIDTransportUSBValue: &'static str = "USB";
pub const kIOHIDTransportBluetoothValue: &'static str = "Bluetooth";
pub const kIOHIDTransportBluetoothLowEnergyValue: &'static str = "BluetoothLowEnergy";
pub const kIOHIDTransportAIDBValue: &'static str = "AIDB";
pub const kIOHIDTransportI2CValue: &'static str = "I2C";
pub const kIOHIDTransportSPIValue: &'static str = "SPI";
pub const kIOHIDTransportSerialValue: &'static str = "Serial";
pub const kIOHIDTransportIAPValue: &'static str = "IAP";
pub const kIOHIDTransportAirPlayValue: &'static str = "AirPlay";
pub const kIOHIDTransportSPUValue: &'static str = "SPU";

pub const kIOHIDCategoryAutomotiveValue: &'static str = "Automotive";

pub const kIOHIDElementKey: &'static str = "Elements";

pub const kIOHIDElementCookieKey: &'static str = "ElementCookie";
pub const kIOHIDElementTypeKey: &'static str = "Type";
pub const kIOHIDElementCollectionTypeKey: &'static str = "CollectionType";
pub const kIOHIDElementUsageKey: &'static str = "Usage";
pub const kIOHIDElementUsagePageKey: &'static str = "UsagePage";
pub const kIOHIDElementMinKey: &'static str = "Min";
pub const kIOHIDElementMaxKey: &'static str = "Max";
pub const kIOHIDElementScaledMinKey: &'static str = "ScaledMin";
pub const kIOHIDElementScaledMaxKey: &'static str = "ScaledMax";
pub const kIOHIDElementSizeKey: &'static str = "Size";
pub const kIOHIDElementReportSizeKey: &'static str = "ReportSize";
pub const kIOHIDElementReportCountKey: &'static str = "ReportCount";
pub const kIOHIDElementReportIDKey: &'static str = "ReportID";
pub const kIOHIDElementIsArrayKey: &'static str = "IsArray";
pub const kIOHIDElementIsRelativeKey: &'static str = "IsRelative";
pub const kIOHIDElementIsWrappingKey: &'static str = "IsWrapping";
pub const kIOHIDElementIsNonLinearKey: &'static str = "IsNonLinear";
pub const kIOHIDElementHasPreferredStateKey: &'static str = "HasPreferredState";
pub const kIOHIDElementHasNullStateKey: &'static str = "HasNullState";
pub const kIOHIDElementFlagsKey: &'static str = "Flags";
pub const kIOHIDElementUnitKey: &'static str = "Unit";
pub const kIOHIDElementUnitExponentKey: &'static str = "UnitExponent";
pub const kIOHIDElementNameKey: &'static str = "Name";
pub const kIOHIDElementValueLocationKey: &'static str = "ValueLocation";
pub const kIOHIDElementDuplicateIndexKey: &'static str = "DuplicateIndex";
pub const kIOHIDElementParentCollectionKey: &'static str = "ParentCollection";

#[cfg(not(target_arch = "powerpc"))]
pub const kIOHIDElementVendorSpecificKey: &'static str = "VendorSpecific";

#[cfg(target_arch = "powerpc")]
pub const kIOHIDElementVendorSpecificKey:&'static str = "VendorSpecifc";

pub const kIOHIDElementCookieMinKey: &'static str = "ElementCookieMin";
pub const kIOHIDElementCookieMaxKey: &'static str = "ElementCookieMax";
pub const kIOHIDElementUsageMinKey: &'static str = "UsageMin";
pub const kIOHIDElementUsageMaxKey: &'static str = "UsageMax";

pub const kIOHIDElementCalibrationMinKey: &'static str = "CalibrationMin";
pub const kIOHIDElementCalibrationMaxKey: &'static str = "CalibrationMax";
pub const kIOHIDElementCalibrationSaturationMinKey: &'static str = "CalibrationSaturationMin";
pub const kIOHIDElementCalibrationSaturationMaxKey: &'static str = "CalibrationSaturationMax";
pub const kIOHIDElementCalibrationDeadZoneMinKey: &'static str = "CalibrationDeadZoneMin";
pub const kIOHIDElementCalibrationDeadZoneMaxKey: &'static str = "CalibrationDeadZoneMax";
pub const kIOHIDElementCalibrationGranularityKey: &'static str = "CalibrationGranularity";

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
