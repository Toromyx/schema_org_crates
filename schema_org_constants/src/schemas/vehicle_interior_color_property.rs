/// <https://schema.org/vehicleInteriorColor>
pub const VEHICLE_INTERIOR_COLOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/vehicleInteriorColor";
/// <https://schema.org/vehicleInteriorColor>
pub const VEHICLE_INTERIOR_COLOR_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/vehicleInteriorColor";
/// <https://schema.org/vehicleInteriorColor>
pub const VEHICLE_INTERIOR_COLOR_PROPERTY_LABEL: &str = "vehicleInteriorColor";
pub struct VehicleInteriorColorPropertyIri;
impl PartialEq<&str> for VehicleInteriorColorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_INTERIOR_COLOR_PROPERTY_IRI_HTTP
			|| *other == VEHICLE_INTERIOR_COLOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleInteriorColorPropertyIri> for &str {
	fn eq(&self, other: &VehicleInteriorColorPropertyIri) -> bool {
		*self == VEHICLE_INTERIOR_COLOR_PROPERTY_IRI_HTTP
			|| *self == VEHICLE_INTERIOR_COLOR_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleInteriorColorPropertyIriOrLabel;
impl PartialEq<&str> for VehicleInteriorColorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleInteriorColorPropertyIri || *other == VEHICLE_INTERIOR_COLOR_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleInteriorColorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleInteriorColorPropertyIriOrLabel) -> bool {
		*self == VehicleInteriorColorPropertyIri || *self == VEHICLE_INTERIOR_COLOR_PROPERTY_LABEL
	}
}
