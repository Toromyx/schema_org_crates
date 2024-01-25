/// <https://schema.org/includesObject>
pub const INCLUDES_OBJECT_PROPERTY_IRI_HTTP: &str = "http://schema.org/includesObject";
/// <https://schema.org/includesObject>
pub const INCLUDES_OBJECT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/includesObject";
/// <https://schema.org/includesObject>
pub const INCLUDES_OBJECT_PROPERTY_LABEL: &str = "includesObject";
pub struct IncludesObjectPropertyIri;
impl PartialEq<&str> for IncludesObjectPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDES_OBJECT_PROPERTY_IRI_HTTP || *other == INCLUDES_OBJECT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludesObjectPropertyIri> for &str {
	fn eq(&self, other: &IncludesObjectPropertyIri) -> bool {
		*self == INCLUDES_OBJECT_PROPERTY_IRI_HTTP || *self == INCLUDES_OBJECT_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludesObjectPropertyIriOrLabel;
impl PartialEq<&str> for IncludesObjectPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludesObjectPropertyIri || *other == INCLUDES_OBJECT_PROPERTY_LABEL
	}
}
impl PartialEq<IncludesObjectPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludesObjectPropertyIriOrLabel) -> bool {
		*self == IncludesObjectPropertyIri || *self == INCLUDES_OBJECT_PROPERTY_LABEL
	}
}
