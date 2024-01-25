/// <https://schema.org/vehicleTransmission>
pub const VEHICLE_TRANSMISSION_PROPERTY_IRI_HTTP: &str = "http://schema.org/vehicleTransmission";
/// <https://schema.org/vehicleTransmission>
pub const VEHICLE_TRANSMISSION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/vehicleTransmission";
/// <https://schema.org/vehicleTransmission>
pub const VEHICLE_TRANSMISSION_PROPERTY_LABEL: &str = "vehicleTransmission";
pub struct VehicleTransmissionPropertyIri;
impl PartialEq<&str> for VehicleTransmissionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_TRANSMISSION_PROPERTY_IRI_HTTP
			|| *other == VEHICLE_TRANSMISSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleTransmissionPropertyIri> for &str {
	fn eq(&self, other: &VehicleTransmissionPropertyIri) -> bool {
		*self == VEHICLE_TRANSMISSION_PROPERTY_IRI_HTTP
			|| *self == VEHICLE_TRANSMISSION_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleTransmissionPropertyIriOrLabel;
impl PartialEq<&str> for VehicleTransmissionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleTransmissionPropertyIri || *other == VEHICLE_TRANSMISSION_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleTransmissionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleTransmissionPropertyIriOrLabel) -> bool {
		*self == VehicleTransmissionPropertyIri || *self == VEHICLE_TRANSMISSION_PROPERTY_LABEL
	}
}
