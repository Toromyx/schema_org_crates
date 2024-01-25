/// <https://schema.org/knownVehicleDamages>
pub const KNOWN_VEHICLE_DAMAGES_PROPERTY_IRI_HTTP: &str = "http://schema.org/knownVehicleDamages";
/// <https://schema.org/knownVehicleDamages>
pub const KNOWN_VEHICLE_DAMAGES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/knownVehicleDamages";
/// <https://schema.org/knownVehicleDamages>
pub const KNOWN_VEHICLE_DAMAGES_PROPERTY_LABEL: &str = "knownVehicleDamages";
pub struct KnownVehicleDamagesPropertyIri;
impl PartialEq<&str> for KnownVehicleDamagesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == KNOWN_VEHICLE_DAMAGES_PROPERTY_IRI_HTTP
			|| *other == KNOWN_VEHICLE_DAMAGES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<KnownVehicleDamagesPropertyIri> for &str {
	fn eq(&self, other: &KnownVehicleDamagesPropertyIri) -> bool {
		*self == KNOWN_VEHICLE_DAMAGES_PROPERTY_IRI_HTTP
			|| *self == KNOWN_VEHICLE_DAMAGES_PROPERTY_IRI_HTTPS
	}
}
pub struct KnownVehicleDamagesPropertyIriOrLabel;
impl PartialEq<&str> for KnownVehicleDamagesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == KnownVehicleDamagesPropertyIri || *other == KNOWN_VEHICLE_DAMAGES_PROPERTY_LABEL
	}
}
impl PartialEq<KnownVehicleDamagesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &KnownVehicleDamagesPropertyIriOrLabel) -> bool {
		*self == KnownVehicleDamagesPropertyIri || *self == KNOWN_VEHICLE_DAMAGES_PROPERTY_LABEL
	}
}
