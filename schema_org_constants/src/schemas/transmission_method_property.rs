/// <https://schema.org/transmissionMethod>
pub const TRANSMISSION_METHOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/transmissionMethod";
/// <https://schema.org/transmissionMethod>
pub const TRANSMISSION_METHOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/transmissionMethod";
/// <https://schema.org/transmissionMethod>
pub const TRANSMISSION_METHOD_PROPERTY_LABEL: &str = "transmissionMethod";
pub struct TransmissionMethodPropertyIri;
impl PartialEq<&str> for TransmissionMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRANSMISSION_METHOD_PROPERTY_IRI_HTTP
			|| *other == TRANSMISSION_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TransmissionMethodPropertyIri> for &str {
	fn eq(&self, other: &TransmissionMethodPropertyIri) -> bool {
		*self == TRANSMISSION_METHOD_PROPERTY_IRI_HTTP
			|| *self == TRANSMISSION_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct TransmissionMethodPropertyIriOrLabel;
impl PartialEq<&str> for TransmissionMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TransmissionMethodPropertyIri || *other == TRANSMISSION_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<TransmissionMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TransmissionMethodPropertyIriOrLabel) -> bool {
		*self == TransmissionMethodPropertyIri || *self == TRANSMISSION_METHOD_PROPERTY_LABEL
	}
}
