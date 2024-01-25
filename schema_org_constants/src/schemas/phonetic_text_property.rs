/// <https://schema.org/phoneticText>
pub const PHONETIC_TEXT_PROPERTY_IRI_HTTP: &str = "http://schema.org/phoneticText";
/// <https://schema.org/phoneticText>
pub const PHONETIC_TEXT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/phoneticText";
/// <https://schema.org/phoneticText>
pub const PHONETIC_TEXT_PROPERTY_LABEL: &str = "phoneticText";
pub struct PhoneticTextPropertyIri;
impl PartialEq<&str> for PhoneticTextPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHONETIC_TEXT_PROPERTY_IRI_HTTP || *other == PHONETIC_TEXT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PhoneticTextPropertyIri> for &str {
	fn eq(&self, other: &PhoneticTextPropertyIri) -> bool {
		*self == PHONETIC_TEXT_PROPERTY_IRI_HTTP || *self == PHONETIC_TEXT_PROPERTY_IRI_HTTPS
	}
}
pub struct PhoneticTextPropertyIriOrLabel;
impl PartialEq<&str> for PhoneticTextPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhoneticTextPropertyIri || *other == PHONETIC_TEXT_PROPERTY_LABEL
	}
}
impl PartialEq<PhoneticTextPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PhoneticTextPropertyIriOrLabel) -> bool {
		*self == PhoneticTextPropertyIri || *self == PHONETIC_TEXT_PROPERTY_LABEL
	}
}
