/// <https://schema.org/FourWheelDriveConfiguration>
pub const FOUR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP: &str =
	"http://schema.org/FourWheelDriveConfiguration";
/// <https://schema.org/FourWheelDriveConfiguration>
pub const FOUR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS: &str =
	"https://schema.org/FourWheelDriveConfiguration";
/// <https://schema.org/FourWheelDriveConfiguration>
pub const FOUR_WHEEL_DRIVE_CONFIGURATION_LABEL: &str = "FourWheelDriveConfiguration";
pub struct FourWheelDriveConfigurationIri;
impl PartialEq<&str> for FourWheelDriveConfigurationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FOUR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
			|| *other == FOUR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
	}
}
impl PartialEq<FourWheelDriveConfigurationIri> for &str {
	fn eq(&self, other: &FourWheelDriveConfigurationIri) -> bool {
		*self == FOUR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
			|| *self == FOUR_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
	}
}
pub struct FourWheelDriveConfigurationIriOrLabel;
impl PartialEq<&str> for FourWheelDriveConfigurationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FourWheelDriveConfigurationIri || *other == FOUR_WHEEL_DRIVE_CONFIGURATION_LABEL
	}
}
impl PartialEq<FourWheelDriveConfigurationIriOrLabel> for &str {
	fn eq(&self, other: &FourWheelDriveConfigurationIriOrLabel) -> bool {
		*self == FourWheelDriveConfigurationIri || *self == FOUR_WHEEL_DRIVE_CONFIGURATION_LABEL
	}
}
