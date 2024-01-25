/// <https://schema.org/pickupTime>
pub const PICKUP_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/pickupTime";
/// <https://schema.org/pickupTime>
pub const PICKUP_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pickupTime";
/// <https://schema.org/pickupTime>
pub const PICKUP_TIME_PROPERTY_LABEL: &str = "pickupTime";
pub struct PickupTimePropertyIri;
impl PartialEq<&str> for PickupTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PICKUP_TIME_PROPERTY_IRI_HTTP || *other == PICKUP_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PickupTimePropertyIri> for &str {
	fn eq(&self, other: &PickupTimePropertyIri) -> bool {
		*self == PICKUP_TIME_PROPERTY_IRI_HTTP || *self == PICKUP_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct PickupTimePropertyIriOrLabel;
impl PartialEq<&str> for PickupTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PickupTimePropertyIri || *other == PICKUP_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<PickupTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PickupTimePropertyIriOrLabel) -> bool {
		*self == PickupTimePropertyIri || *self == PICKUP_TIME_PROPERTY_LABEL
	}
}
