/// <https://schema.org/repeatCount>
pub const REPEAT_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/repeatCount";
/// <https://schema.org/repeatCount>
pub const REPEAT_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/repeatCount";
/// <https://schema.org/repeatCount>
pub const REPEAT_COUNT_PROPERTY_LABEL: &str = "repeatCount";
pub struct RepeatCountPropertyIri;
impl PartialEq<&str> for RepeatCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPEAT_COUNT_PROPERTY_IRI_HTTP || *other == REPEAT_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RepeatCountPropertyIri> for &str {
	fn eq(&self, other: &RepeatCountPropertyIri) -> bool {
		*self == REPEAT_COUNT_PROPERTY_IRI_HTTP || *self == REPEAT_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct RepeatCountPropertyIriOrLabel;
impl PartialEq<&str> for RepeatCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RepeatCountPropertyIri || *other == REPEAT_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<RepeatCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RepeatCountPropertyIriOrLabel) -> bool {
		*self == RepeatCountPropertyIri || *self == REPEAT_COUNT_PROPERTY_LABEL
	}
}
