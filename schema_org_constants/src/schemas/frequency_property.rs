/// <https://schema.org/frequency>
pub const FREQUENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/frequency";
/// <https://schema.org/frequency>
pub const FREQUENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/frequency";
/// <https://schema.org/frequency>
pub const FREQUENCY_PROPERTY_LABEL: &str = "frequency";
pub struct FrequencyPropertyIri;
impl PartialEq<&str> for FrequencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FREQUENCY_PROPERTY_IRI_HTTP || *other == FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FrequencyPropertyIri> for &str {
	fn eq(&self, other: &FrequencyPropertyIri) -> bool {
		*self == FREQUENCY_PROPERTY_IRI_HTTP || *self == FREQUENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct FrequencyPropertyIriOrLabel;
impl PartialEq<&str> for FrequencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FrequencyPropertyIri || *other == FREQUENCY_PROPERTY_LABEL
	}
}
impl PartialEq<FrequencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FrequencyPropertyIriOrLabel) -> bool {
		*self == FrequencyPropertyIri || *self == FREQUENCY_PROPERTY_LABEL
	}
}
