/// <https://schema.org/nextItem>
pub const NEXT_ITEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/nextItem";
/// <https://schema.org/nextItem>
pub const NEXT_ITEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nextItem";
/// <https://schema.org/nextItem>
pub const NEXT_ITEM_PROPERTY_LABEL: &str = "nextItem";
pub struct NextItemPropertyIri;
impl PartialEq<&str> for NextItemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEXT_ITEM_PROPERTY_IRI_HTTP || *other == NEXT_ITEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NextItemPropertyIri> for &str {
	fn eq(&self, other: &NextItemPropertyIri) -> bool {
		*self == NEXT_ITEM_PROPERTY_IRI_HTTP || *self == NEXT_ITEM_PROPERTY_IRI_HTTPS
	}
}
pub struct NextItemPropertyIriOrLabel;
impl PartialEq<&str> for NextItemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NextItemPropertyIri || *other == NEXT_ITEM_PROPERTY_LABEL
	}
}
impl PartialEq<NextItemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NextItemPropertyIriOrLabel) -> bool {
		*self == NextItemPropertyIri || *self == NEXT_ITEM_PROPERTY_LABEL
	}
}
