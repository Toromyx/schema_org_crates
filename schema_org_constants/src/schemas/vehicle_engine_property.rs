/// <https://schema.org/vehicleEngine>
pub const VEHICLE_ENGINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/vehicleEngine";
/// <https://schema.org/vehicleEngine>
pub const VEHICLE_ENGINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/vehicleEngine";
/// <https://schema.org/vehicleEngine>
pub const VEHICLE_ENGINE_PROPERTY_LABEL: &str = "vehicleEngine";
pub struct VehicleEnginePropertyIri;
impl PartialEq<&str> for VehicleEnginePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VEHICLE_ENGINE_PROPERTY_IRI_HTTP || *other == VEHICLE_ENGINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VehicleEnginePropertyIri> for &str {
	fn eq(&self, other: &VehicleEnginePropertyIri) -> bool {
		*self == VEHICLE_ENGINE_PROPERTY_IRI_HTTP || *self == VEHICLE_ENGINE_PROPERTY_IRI_HTTPS
	}
}
pub struct VehicleEnginePropertyIriOrLabel;
impl PartialEq<&str> for VehicleEnginePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VehicleEnginePropertyIri || *other == VEHICLE_ENGINE_PROPERTY_LABEL
	}
}
impl PartialEq<VehicleEnginePropertyIriOrLabel> for &str {
	fn eq(&self, other: &VehicleEnginePropertyIriOrLabel) -> bool {
		*self == VehicleEnginePropertyIri || *self == VEHICLE_ENGINE_PROPERTY_LABEL
	}
}
