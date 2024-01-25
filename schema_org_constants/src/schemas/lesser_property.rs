/// <https://schema.org/lesser>
pub const LESSER_PROPERTY_IRI_HTTP: &str = "http://schema.org/lesser";
/// <https://schema.org/lesser>
pub const LESSER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/lesser";
/// <https://schema.org/lesser>
pub const LESSER_PROPERTY_LABEL: &str = "lesser";
pub struct LesserPropertyIri;
impl PartialEq<&str> for LesserPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LESSER_PROPERTY_IRI_HTTP || *other == LESSER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LesserPropertyIri> for &str {
	fn eq(&self, other: &LesserPropertyIri) -> bool {
		*self == LESSER_PROPERTY_IRI_HTTP || *self == LESSER_PROPERTY_IRI_HTTPS
	}
}
pub struct LesserPropertyIriOrLabel;
impl PartialEq<&str> for LesserPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LesserPropertyIri || *other == LESSER_PROPERTY_LABEL
	}
}
impl PartialEq<LesserPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LesserPropertyIriOrLabel) -> bool {
		*self == LesserPropertyIri || *self == LESSER_PROPERTY_LABEL
	}
}
