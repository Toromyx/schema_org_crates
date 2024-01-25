/// <https://schema.org/FrontWheelDriveConfiguration>
pub const FRONT_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP: &str =
	"http://schema.org/FrontWheelDriveConfiguration";
/// <https://schema.org/FrontWheelDriveConfiguration>
pub const FRONT_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS: &str =
	"https://schema.org/FrontWheelDriveConfiguration";
/// <https://schema.org/FrontWheelDriveConfiguration>
pub const FRONT_WHEEL_DRIVE_CONFIGURATION_LABEL: &str = "FrontWheelDriveConfiguration";
pub struct FrontWheelDriveConfigurationIri;
impl PartialEq<&str> for FrontWheelDriveConfigurationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FRONT_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
			|| *other == FRONT_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
	}
}
impl PartialEq<FrontWheelDriveConfigurationIri> for &str {
	fn eq(&self, other: &FrontWheelDriveConfigurationIri) -> bool {
		*self == FRONT_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
			|| *self == FRONT_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
	}
}
pub struct FrontWheelDriveConfigurationIriOrLabel;
impl PartialEq<&str> for FrontWheelDriveConfigurationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FrontWheelDriveConfigurationIri || *other == FRONT_WHEEL_DRIVE_CONFIGURATION_LABEL
	}
}
impl PartialEq<FrontWheelDriveConfigurationIriOrLabel> for &str {
	fn eq(&self, other: &FrontWheelDriveConfigurationIriOrLabel) -> bool {
		*self == FrontWheelDriveConfigurationIri || *self == FRONT_WHEEL_DRIVE_CONFIGURATION_LABEL
	}
}
