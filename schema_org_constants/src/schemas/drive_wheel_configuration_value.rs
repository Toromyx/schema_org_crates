/// <https://schema.org/DriveWheelConfigurationValue>
pub const DRIVE_WHEEL_CONFIGURATION_VALUE_IRI_HTTP: &str =
	"http://schema.org/DriveWheelConfigurationValue";
/// <https://schema.org/DriveWheelConfigurationValue>
pub const DRIVE_WHEEL_CONFIGURATION_VALUE_IRI_HTTPS: &str =
	"https://schema.org/DriveWheelConfigurationValue";
/// <https://schema.org/DriveWheelConfigurationValue>
pub const DRIVE_WHEEL_CONFIGURATION_VALUE_LABEL: &str = "DriveWheelConfigurationValue";
pub struct DriveWheelConfigurationValueIri;
impl PartialEq<&str> for DriveWheelConfigurationValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRIVE_WHEEL_CONFIGURATION_VALUE_IRI_HTTP
			|| *other == DRIVE_WHEEL_CONFIGURATION_VALUE_IRI_HTTPS
	}
}
impl PartialEq<DriveWheelConfigurationValueIri> for &str {
	fn eq(&self, other: &DriveWheelConfigurationValueIri) -> bool {
		*self == DRIVE_WHEEL_CONFIGURATION_VALUE_IRI_HTTP
			|| *self == DRIVE_WHEEL_CONFIGURATION_VALUE_IRI_HTTPS
	}
}
pub struct DriveWheelConfigurationValueIriOrLabel;
impl PartialEq<&str> for DriveWheelConfigurationValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DriveWheelConfigurationValueIri || *other == DRIVE_WHEEL_CONFIGURATION_VALUE_LABEL
	}
}
impl PartialEq<DriveWheelConfigurationValueIriOrLabel> for &str {
	fn eq(&self, other: &DriveWheelConfigurationValueIriOrLabel) -> bool {
		*self == DriveWheelConfigurationValueIri || *self == DRIVE_WHEEL_CONFIGURATION_VALUE_LABEL
	}
}
