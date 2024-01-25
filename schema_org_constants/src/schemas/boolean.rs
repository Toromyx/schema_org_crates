/// <https://schema.org/Boolean>
pub const BOOLEAN_IRI_HTTP: &str = "http://schema.org/Boolean";
/// <https://schema.org/Boolean>
pub const BOOLEAN_IRI_HTTPS: &str = "https://schema.org/Boolean";
/// <https://schema.org/Boolean>
pub const BOOLEAN_LABEL: &str = "Boolean";
pub struct BooleanIri;
impl PartialEq<&str> for BooleanIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOLEAN_IRI_HTTP || *other == BOOLEAN_IRI_HTTPS
	}
}
impl PartialEq<BooleanIri> for &str {
	fn eq(&self, other: &BooleanIri) -> bool {
		*self == BOOLEAN_IRI_HTTP || *self == BOOLEAN_IRI_HTTPS
	}
}
pub struct BooleanIriOrLabel;
impl PartialEq<&str> for BooleanIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BooleanIri || *other == BOOLEAN_LABEL
	}
}
impl PartialEq<BooleanIriOrLabel> for &str {
	fn eq(&self, other: &BooleanIriOrLabel) -> bool {
		*self == BooleanIri || *self == BOOLEAN_LABEL
	}
}
