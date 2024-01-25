/// <https://schema.org/vehicleInteriorType>
pub const VEHICLE_INTERIOR_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/vehicleInteriorType";
/// <https://schema.org/vehicleInteriorType>
pub const VEHICLE_INTERIOR_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/vehicleInteriorType";
/// <https://schema.org/vehicleInteriorType>
pub const VEHICLE_INTERIOR_TYPE_PROPERTY_LABEL: &str = "vehicleInteriorType";
pub struct VehicleInteriorTypePropertyIri;
impl PartialEq<&str> for VehicleInteriorTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_INTERIOR_TYPE_PROPERTY_IRI_HTTP
			|| *other == VEHICLE_INTERIOR_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleInteriorTypePropertyIri> for &str {
	fn eq(&self, other: &VehicleInteriorTypePropertyIri) -> bool {
		*self == VEHICLE_INTERIOR_TYPE_PROPERTY_IRI_HTTP
			|| *self == VEHICLE_INTERIOR_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleInteriorTypePropertyIriOrLabel;
impl PartialEq<&str> for VehicleInteriorTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleInteriorTypePropertyIri || *other == VEHICLE_INTERIOR_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleInteriorTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleInteriorTypePropertyIriOrLabel) -> bool {
		*self == VehicleInteriorTypePropertyIri || *self == VEHICLE_INTERIOR_TYPE_PROPERTY_LABEL
	}
}
