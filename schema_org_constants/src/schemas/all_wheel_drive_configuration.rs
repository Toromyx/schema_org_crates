/// <https://schema.org/AllWheelDriveConfiguration>
pub const ALL_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP: &str =
	"http://schema.org/AllWheelDriveConfiguration";
/// <https://schema.org/AllWheelDriveConfiguration>
pub const ALL_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS: &str =
	"https://schema.org/AllWheelDriveConfiguration";
/// <https://schema.org/AllWheelDriveConfiguration>
pub const ALL_WHEEL_DRIVE_CONFIGURATION_LABEL: &str = "AllWheelDriveConfiguration";
pub struct AllWheelDriveConfigurationIri;
impl PartialEq<&str> for AllWheelDriveConfigurationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALL_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
			|| *other == ALL_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
	}
}
impl PartialEq<AllWheelDriveConfigurationIri> for &str {
	fn eq(&self, other: &AllWheelDriveConfigurationIri) -> bool {
		*self == ALL_WHEEL_DRIVE_CONFIGURATION_IRI_HTTP
			|| *self == ALL_WHEEL_DRIVE_CONFIGURATION_IRI_HTTPS
	}
}
pub struct AllWheelDriveConfigurationIriOrLabel;
impl PartialEq<&str> for AllWheelDriveConfigurationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AllWheelDriveConfigurationIri || *other == ALL_WHEEL_DRIVE_CONFIGURATION_LABEL
	}
}
impl PartialEq<AllWheelDriveConfigurationIriOrLabel> for &str {
	fn eq(&self, other: &AllWheelDriveConfigurationIriOrLabel) -> bool {
		*self == AllWheelDriveConfigurationIri || *self == ALL_WHEEL_DRIVE_CONFIGURATION_LABEL
	}
}
