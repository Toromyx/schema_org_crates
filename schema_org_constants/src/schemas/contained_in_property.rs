/// <https://schema.org/containedIn>
#[deprecated = "This schema is superseded by <https://schema.org/containedInPlace>."]
pub const CONTAINED_IN_PROPERTY_IRI_HTTP: &str = "http://schema.org/containedIn";
/// <https://schema.org/containedIn>
#[deprecated = "This schema is superseded by <https://schema.org/containedInPlace>."]
pub const CONTAINED_IN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/containedIn";
/// <https://schema.org/containedIn>
#[deprecated = "This schema is superseded by <https://schema.org/containedInPlace>."]
pub const CONTAINED_IN_PROPERTY_LABEL: &str = "containedIn";
pub struct ContainedInPropertyIri;
impl PartialEq<&str> for ContainedInPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTAINED_IN_PROPERTY_IRI_HTTP || *other == CONTAINED_IN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContainedInPropertyIri> for &str {
	fn eq(&self, other: &ContainedInPropertyIri) -> bool {
		*self == CONTAINED_IN_PROPERTY_IRI_HTTP || *self == CONTAINED_IN_PROPERTY_IRI_HTTPS
	}
}
pub struct ContainedInPropertyIriOrLabel;
impl PartialEq<&str> for ContainedInPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContainedInPropertyIri || *other == CONTAINED_IN_PROPERTY_LABEL
	}
}
impl PartialEq<ContainedInPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContainedInPropertyIriOrLabel) -> bool {
		*self == ContainedInPropertyIri || *self == CONTAINED_IN_PROPERTY_LABEL
	}
}
