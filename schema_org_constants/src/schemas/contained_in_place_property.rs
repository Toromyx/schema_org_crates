/// <https://schema.org/containedInPlace>
pub const CONTAINED_IN_PLACE_PROPERTY_IRI_HTTP: &str = "http://schema.org/containedInPlace";
/// <https://schema.org/containedInPlace>
pub const CONTAINED_IN_PLACE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/containedInPlace";
/// <https://schema.org/containedInPlace>
pub const CONTAINED_IN_PLACE_PROPERTY_LABEL: &str = "containedInPlace";
pub struct ContainedInPlacePropertyIri;
impl PartialEq<&str> for ContainedInPlacePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTAINED_IN_PLACE_PROPERTY_IRI_HTTP
			|| *other == CONTAINED_IN_PLACE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContainedInPlacePropertyIri> for &str {
	fn eq(&self, other: &ContainedInPlacePropertyIri) -> bool {
		*self == CONTAINED_IN_PLACE_PROPERTY_IRI_HTTP
			|| *self == CONTAINED_IN_PLACE_PROPERTY_IRI_HTTPS
	}
}
pub struct ContainedInPlacePropertyIriOrLabel;
impl PartialEq<&str> for ContainedInPlacePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContainedInPlacePropertyIri || *other == CONTAINED_IN_PLACE_PROPERTY_LABEL
	}
}
impl PartialEq<ContainedInPlacePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContainedInPlacePropertyIriOrLabel) -> bool {
		*self == ContainedInPlacePropertyIri || *self == CONTAINED_IN_PLACE_PROPERTY_LABEL
	}
}
