/// <https://schema.org/sameAs>
pub const SAME_AS_PROPERTY_IRI_HTTP: &str = "http://schema.org/sameAs";
/// <https://schema.org/sameAs>
pub const SAME_AS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sameAs";
/// <https://schema.org/sameAs>
pub const SAME_AS_PROPERTY_LABEL: &str = "sameAs";
pub struct SameAsPropertyIri;
impl PartialEq<&str> for SameAsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SAME_AS_PROPERTY_IRI_HTTP || *other == SAME_AS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SameAsPropertyIri> for &str {
	fn eq(&self, other: &SameAsPropertyIri) -> bool {
		*self == SAME_AS_PROPERTY_IRI_HTTP || *self == SAME_AS_PROPERTY_IRI_HTTPS
	}
}
pub struct SameAsPropertyIriOrLabel;
impl PartialEq<&str> for SameAsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SameAsPropertyIri || *other == SAME_AS_PROPERTY_LABEL
	}
}
impl PartialEq<SameAsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SameAsPropertyIriOrLabel) -> bool {
		*self == SameAsPropertyIri || *self == SAME_AS_PROPERTY_LABEL
	}
}
