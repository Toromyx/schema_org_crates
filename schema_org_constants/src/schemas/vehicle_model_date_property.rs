/// <https://schema.org/vehicleModelDate>
pub const VEHICLE_MODEL_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/vehicleModelDate";
/// <https://schema.org/vehicleModelDate>
pub const VEHICLE_MODEL_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/vehicleModelDate";
/// <https://schema.org/vehicleModelDate>
pub const VEHICLE_MODEL_DATE_PROPERTY_LABEL: &str = "vehicleModelDate";
pub struct VehicleModelDatePropertyIri;
impl PartialEq<&str> for VehicleModelDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_MODEL_DATE_PROPERTY_IRI_HTTP
			|| *other == VEHICLE_MODEL_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleModelDatePropertyIri> for &str {
	fn eq(&self, other: &VehicleModelDatePropertyIri) -> bool {
		*self == VEHICLE_MODEL_DATE_PROPERTY_IRI_HTTP
			|| *self == VEHICLE_MODEL_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleModelDatePropertyIriOrLabel;
impl PartialEq<&str> for VehicleModelDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleModelDatePropertyIri || *other == VEHICLE_MODEL_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleModelDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleModelDatePropertyIriOrLabel) -> bool {
		*self == VehicleModelDatePropertyIri || *self == VEHICLE_MODEL_DATE_PROPERTY_LABEL
	}
}
