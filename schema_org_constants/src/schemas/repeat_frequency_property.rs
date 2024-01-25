/// <https://schema.org/repeatFrequency>
pub const REPEAT_FREQUENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/repeatFrequency";
/// <https://schema.org/repeatFrequency>
pub const REPEAT_FREQUENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/repeatFrequency";
/// <https://schema.org/repeatFrequency>
pub const REPEAT_FREQUENCY_PROPERTY_LABEL: &str = "repeatFrequency";
pub struct RepeatFrequencyPropertyIri;
impl PartialEq<&str> for RepeatFrequencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPEAT_FREQUENCY_PROPERTY_IRI_HTTP
			|| *other == REPEAT_FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RepeatFrequencyPropertyIri> for &str {
	fn eq(&self, other: &RepeatFrequencyPropertyIri) -> bool {
		*self == REPEAT_FREQUENCY_PROPERTY_IRI_HTTP || *self == REPEAT_FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct RepeatFrequencyPropertyIriOrLabel;
impl PartialEq<&str> for RepeatFrequencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RepeatFrequencyPropertyIri || *other == REPEAT_FREQUENCY_PROPERTY_LABEL
	}
}
impl PartialEq<RepeatFrequencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RepeatFrequencyPropertyIriOrLabel) -> bool {
		*self == RepeatFrequencyPropertyIri || *self == REPEAT_FREQUENCY_PROPERTY_LABEL
	}
}
