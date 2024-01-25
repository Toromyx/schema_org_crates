/// <https://schema.org/vehicleConfiguration>
pub const VEHICLE_CONFIGURATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/vehicleConfiguration";
/// <https://schema.org/vehicleConfiguration>
pub const VEHICLE_CONFIGURATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/vehicleConfiguration";
/// <https://schema.org/vehicleConfiguration>
pub const VEHICLE_CONFIGURATION_PROPERTY_LABEL: &str = "vehicleConfiguration";
pub struct VehicleConfigurationPropertyIri;
impl PartialEq<&str> for VehicleConfigurationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_CONFIGURATION_PROPERTY_IRI_HTTP
			|| *other == VEHICLE_CONFIGURATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleConfigurationPropertyIri> for &str {
	fn eq(&self, other: &VehicleConfigurationPropertyIri) -> bool {
		*self == VEHICLE_CONFIGURATION_PROPERTY_IRI_HTTP
			|| *self == VEHICLE_CONFIGURATION_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleConfigurationPropertyIriOrLabel;
impl PartialEq<&str> for VehicleConfigurationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleConfigurationPropertyIri || *other == VEHICLE_CONFIGURATION_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleConfigurationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleConfigurationPropertyIriOrLabel) -> bool {
		*self == VehicleConfigurationPropertyIri || *self == VEHICLE_CONFIGURATION_PROPERTY_LABEL
	}
}
