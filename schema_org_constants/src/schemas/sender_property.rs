/// <https://schema.org/sender>
pub const SENDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/sender";
/// <https://schema.org/sender>
pub const SENDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sender";
/// <https://schema.org/sender>
pub const SENDER_PROPERTY_LABEL: &str = "sender";
pub struct SenderPropertyIri;
impl PartialEq<&str> for SenderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SENDER_PROPERTY_IRI_HTTP || *other == SENDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SenderPropertyIri> for &str {
	fn eq(&self, other: &SenderPropertyIri) -> bool {
		*self == SENDER_PROPERTY_IRI_HTTP || *self == SENDER_PROPERTY_IRI_HTTPS
	}
}
pub struct SenderPropertyIriOrLabel;
impl PartialEq<&str> for SenderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SenderPropertyIri || *other == SENDER_PROPERTY_LABEL
	}
}
impl PartialEq<SenderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SenderPropertyIriOrLabel) -> bool {
		*self == SenderPropertyIri || *self == SENDER_PROPERTY_LABEL
	}
}
