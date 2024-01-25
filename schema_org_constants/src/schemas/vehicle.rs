/// <https://schema.org/Vehicle>
pub const VEHICLE_IRI_HTTP: &str = "http://schema.org/Vehicle";
/// <https://schema.org/Vehicle>
pub const VEHICLE_IRI_HTTPS: &str = "https://schema.org/Vehicle";
/// <https://schema.org/Vehicle>
pub const VEHICLE_LABEL: &str = "Vehicle";
pub struct VehicleIri;
impl PartialEq<&str> for VehicleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_IRI_HTTP || *other == VEHICLE_IRI_HTTPS
	}
}
impl PartialEq<VehicleIri> for &str {
	fn eq(&self, other: &VehicleIri) -> bool {
		*self == VEHICLE_IRI_HTTP || *self == VEHICLE_IRI_HTTPS
	}
}
pub struct VehicleIriOrLabel;
impl PartialEq<&str> for VehicleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleIri || *other == VEHICLE_LABEL
	}
}
impl PartialEq<VehicleIriOrLabel> for &str {
	fn eq(&self, other: &VehicleIriOrLabel) -> bool {
		*self == VehicleIri || *self == VEHICLE_LABEL
	}
}
