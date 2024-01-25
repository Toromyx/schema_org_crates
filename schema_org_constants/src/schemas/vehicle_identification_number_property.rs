/// <https://schema.org/vehicleIdentificationNumber>
pub const VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/vehicleIdentificationNumber";
/// <https://schema.org/vehicleIdentificationNumber>
pub const VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/vehicleIdentificationNumber";
/// <https://schema.org/vehicleIdentificationNumber>
pub const VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_LABEL: &str = "vehicleIdentificationNumber";
pub struct VehicleIdentificationNumberPropertyIri;
impl PartialEq<&str> for VehicleIdentificationNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_IRI_HTTP
			|| *other == VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleIdentificationNumberPropertyIri> for &str {
	fn eq(&self, other: &VehicleIdentificationNumberPropertyIri) -> bool {
		*self == VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_IRI_HTTP
			|| *self == VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleIdentificationNumberPropertyIriOrLabel;
impl PartialEq<&str> for VehicleIdentificationNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleIdentificationNumberPropertyIri
			|| *other == VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleIdentificationNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleIdentificationNumberPropertyIriOrLabel) -> bool {
		*self == VehicleIdentificationNumberPropertyIri
			|| *self == VEHICLE_IDENTIFICATION_NUMBER_PROPERTY_LABEL
	}
}
