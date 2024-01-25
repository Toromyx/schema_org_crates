/// <https://schema.org/pickupLocation>
pub const PICKUP_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/pickupLocation";
/// <https://schema.org/pickupLocation>
pub const PICKUP_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pickupLocation";
/// <https://schema.org/pickupLocation>
pub const PICKUP_LOCATION_PROPERTY_LABEL: &str = "pickupLocation";
pub struct PickupLocationPropertyIri;
impl PartialEq<&str> for PickupLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PICKUP_LOCATION_PROPERTY_IRI_HTTP || *other == PICKUP_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PickupLocationPropertyIri> for &str {
	fn eq(&self, other: &PickupLocationPropertyIri) -> bool {
		*self == PICKUP_LOCATION_PROPERTY_IRI_HTTP || *self == PICKUP_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct PickupLocationPropertyIriOrLabel;
impl PartialEq<&str> for PickupLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PickupLocationPropertyIri || *other == PICKUP_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<PickupLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PickupLocationPropertyIriOrLabel) -> bool {
		*self == PickupLocationPropertyIri || *self == PICKUP_LOCATION_PROPERTY_LABEL
	}
}
