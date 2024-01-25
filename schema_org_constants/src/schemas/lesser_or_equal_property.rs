/// <https://schema.org/lesserOrEqual>
pub const LESSER_OR_EQUAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/lesserOrEqual";
/// <https://schema.org/lesserOrEqual>
pub const LESSER_OR_EQUAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/lesserOrEqual";
/// <https://schema.org/lesserOrEqual>
pub const LESSER_OR_EQUAL_PROPERTY_LABEL: &str = "lesserOrEqual";
pub struct LesserOrEqualPropertyIri;
impl PartialEq<&str> for LesserOrEqualPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LESSER_OR_EQUAL_PROPERTY_IRI_HTTP || *other == LESSER_OR_EQUAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LesserOrEqualPropertyIri> for &str {
	fn eq(&self, other: &LesserOrEqualPropertyIri) -> bool {
		*self == LESSER_OR_EQUAL_PROPERTY_IRI_HTTP || *self == LESSER_OR_EQUAL_PROPERTY_IRI_HTTPS
	}
}
pub struct LesserOrEqualPropertyIriOrLabel;
impl PartialEq<&str> for LesserOrEqualPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LesserOrEqualPropertyIri || *other == LESSER_OR_EQUAL_PROPERTY_LABEL
	}
}
impl PartialEq<LesserOrEqualPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LesserOrEqualPropertyIriOrLabel) -> bool {
		*self == LesserOrEqualPropertyIri || *self == LESSER_OR_EQUAL_PROPERTY_LABEL
	}
}
