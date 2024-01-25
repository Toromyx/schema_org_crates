/// <https://schema.org/producer>
pub const PRODUCER_PROPERTY_IRI_HTTP: &str = "http://schema.org/producer";
/// <https://schema.org/producer>
pub const PRODUCER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/producer";
/// <https://schema.org/producer>
pub const PRODUCER_PROPERTY_LABEL: &str = "producer";
pub struct ProducerPropertyIri;
impl PartialEq<&str> for ProducerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCER_PROPERTY_IRI_HTTP || *other == PRODUCER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProducerPropertyIri> for &str {
	fn eq(&self, other: &ProducerPropertyIri) -> bool {
		*self == PRODUCER_PROPERTY_IRI_HTTP || *self == PRODUCER_PROPERTY_IRI_HTTPS
	}
}
pub struct ProducerPropertyIriOrLabel;
impl PartialEq<&str> for ProducerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProducerPropertyIri || *other == PRODUCER_PROPERTY_LABEL
	}
}
impl PartialEq<ProducerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProducerPropertyIriOrLabel) -> bool {
		*self == ProducerPropertyIri || *self == PRODUCER_PROPERTY_LABEL
	}
}
