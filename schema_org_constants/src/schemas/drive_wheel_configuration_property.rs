/// <https://schema.org/driveWheelConfiguration>
pub const DRIVE_WHEEL_CONFIGURATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/driveWheelConfiguration";
/// <https://schema.org/driveWheelConfiguration>
pub const DRIVE_WHEEL_CONFIGURATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/driveWheelConfiguration";
/// <https://schema.org/driveWheelConfiguration>
pub const DRIVE_WHEEL_CONFIGURATION_PROPERTY_LABEL: &str = "driveWheelConfiguration";
pub struct DriveWheelConfigurationPropertyIri;
impl PartialEq<&str> for DriveWheelConfigurationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRIVE_WHEEL_CONFIGURATION_PROPERTY_IRI_HTTP
			|| *other == DRIVE_WHEEL_CONFIGURATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DriveWheelConfigurationPropertyIri> for &str {
	fn eq(&self, other: &DriveWheelConfigurationPropertyIri) -> bool {
		*self == DRIVE_WHEEL_CONFIGURATION_PROPERTY_IRI_HTTP
			|| *self == DRIVE_WHEEL_CONFIGURATION_PROPERTY_IRI_HTTPS
	}
}
pub struct DriveWheelConfigurationPropertyIriOrLabel;
impl PartialEq<&str> for DriveWheelConfigurationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DriveWheelConfigurationPropertyIri
			|| *other == DRIVE_WHEEL_CONFIGURATION_PROPERTY_LABEL
	}
}
impl PartialEq<DriveWheelConfigurationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DriveWheelConfigurationPropertyIriOrLabel) -> bool {
		*self == DriveWheelConfigurationPropertyIri
			|| *self == DRIVE_WHEEL_CONFIGURATION_PROPERTY_LABEL
	}
}
