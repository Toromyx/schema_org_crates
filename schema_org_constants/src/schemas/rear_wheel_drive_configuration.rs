/// <https://schema.org/RearWheelDriveConfiguration>
pub const REAR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP: &str =
	"http://schema.org/RearWheelDriveConfiguration";
/// <https://schema.org/RearWheelDriveConfiguration>
pub const REAR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS: &str =
	"https://schema.org/RearWheelDriveConfiguration";
/// <https://schema.org/RearWheelDriveConfiguration>
pub const REAR_WHEEL_DRIVE_CONFIGURATION_LABEL: &str = "RearWheelDriveConfiguration";
pub struct RearWheelDriveConfigurationIri;
impl PartialEq<&str> for RearWheelDriveConfigurationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REAR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
			|| *other == REAR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
	}
}
impl PartialEq<RearWheelDriveConfigurationIri> for &str {
	fn eq(&self, other: &RearWheelDriveConfigurationIri) -> bool {
		*self == REAR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
			|| *self == REAR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
	}
}
pub struct RearWheelDriveConfigurationIriOrLabel;
impl PartialEq<&str> for RearWheelDriveConfigurationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RearWheelDriveConfigurationIri || *other == REAR_WHEEL_DRIVE_CONFIGURATION_LABEL
	}
}
impl PartialEq<RearWheelDriveConfigurationIriOrLabel> for &str {
	fn eq(&self, other: &RearWheelDriveConfigurationIriOrLabel) -> bool {
		*self == RearWheelDriveConfigurationIri || *self == REAR_WHEEL_DRIVE_CONFIGURATION_LABEL
	}
}
