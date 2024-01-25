/// <https://schema.org/containsPlace>
pub const CONTAINS_PLACE_PROPERTY_IRI_HTTP: &str = "http://schema.org/containsPlace";
/// <https://schema.org/containsPlace>
pub const CONTAINS_PLACE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/containsPlace";
/// <https://schema.org/containsPlace>
pub const CONTAINS_PLACE_PROPERTY_LABEL: &str = "containsPlace";
pub struct ContainsPlacePropertyIri;
impl PartialEq<&str> for ContainsPlacePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTAINS_PLACE_PROPERTY_IRI_HTTP || *other == CONTAINS_PLACE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContainsPlacePropertyIri> for &str {
	fn eq(&self, other: &ContainsPlacePropertyIri) -> bool {
		*self == CONTAINS_PLACE_PROPERTY_IRI_HTTP || *self == CONTAINS_PLACE_PROPERTY_IRI_HTTPS
	}
}
pub struct ContainsPlacePropertyIriOrLabel;
impl PartialEq<&str> for ContainsPlacePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContainsPlacePropertyIri || *other == CONTAINS_PLACE_PROPERTY_LABEL
	}
}
impl PartialEq<ContainsPlacePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContainsPlacePropertyIriOrLabel) -> bool {
		*self == ContainsPlacePropertyIri || *self == CONTAINS_PLACE_PROPERTY_LABEL
	}
}
