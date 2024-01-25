/// <https://schema.org/ItemAvailability>
pub const ITEM_AVAILABILITY_IRI_HTTP: &str = "http://schema.org/ItemAvailability";
/// <https://schema.org/ItemAvailability>
pub const ITEM_AVAILABILITY_IRI_HTTPS: &str = "https://schema.org/ItemAvailability";
/// <https://schema.org/ItemAvailability>
pub const ITEM_AVAILABILITY_LABEL: &str = "ItemAvailability";
pub struct ItemAvailabilityIri;
impl PartialEq<&str> for ItemAvailabilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ITEM_AVAILABILITY_IRI_HTTP || *other == ITEM_AVAILABILITY_IRI_HTTPS
	}
}
impl PartialEq<ItemAvailabilityIri> for &str {
	fn eq(&self, other: &ItemAvailabilityIri) -> bool {
		*self == ITEM_AVAILABILITY_IRI_HTTP || *self == ITEM_AVAILABILITY_IRI_HTTPS
	}
}
pub struct ItemAvailabilityIriOrLabel;
impl PartialEq<&str> for ItemAvailabilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ItemAvailabilityIri || *other == ITEM_AVAILABILITY_LABEL
	}
}
impl PartialEq<ItemAvailabilityIriOrLabel> for &str {
	fn eq(&self, other: &ItemAvailabilityIriOrLabel) -> bool {
		*self == ItemAvailabilityIri || *self == ITEM_AVAILABILITY_LABEL
	}
}
