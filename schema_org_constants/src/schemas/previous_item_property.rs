/// <https://schema.org/previousItem>
pub const PREVIOUS_ITEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/previousItem";
/// <https://schema.org/previousItem>
pub const PREVIOUS_ITEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/previousItem";
/// <https://schema.org/previousItem>
pub const PREVIOUS_ITEM_PROPERTY_LABEL: &str = "previousItem";
pub struct PreviousItemPropertyIri;
impl PartialEq<&str> for PreviousItemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREVIOUS_ITEM_PROPERTY_IRI_HTTP || *other == PREVIOUS_ITEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PreviousItemPropertyIri> for &str {
	fn eq(&self, other: &PreviousItemPropertyIri) -> bool {
		*self == PREVIOUS_ITEM_PROPERTY_IRI_HTTP || *self == PREVIOUS_ITEM_PROPERTY_IRI_HTTPS
	}
}
pub struct PreviousItemPropertyIriOrLabel;
impl PartialEq<&str> for PreviousItemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreviousItemPropertyIri || *other == PREVIOUS_ITEM_PROPERTY_LABEL
	}
}
impl PartialEq<PreviousItemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PreviousItemPropertyIriOrLabel) -> bool {
		*self == PreviousItemPropertyIri || *self == PREVIOUS_ITEM_PROPERTY_LABEL
	}
}
