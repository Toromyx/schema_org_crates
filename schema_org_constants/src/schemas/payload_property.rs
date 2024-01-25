/// <https://schema.org/payload>
pub const PAYLOAD_PROPERTY_IRI_HTTP: &str = "http://schema.org/payload";
/// <https://schema.org/payload>
pub const PAYLOAD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/payload";
/// <https://schema.org/payload>
pub const PAYLOAD_PROPERTY_LABEL: &str = "payload";
pub struct PayloadPropertyIri;
impl PartialEq<&str> for PayloadPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAYLOAD_PROPERTY_IRI_HTTP || *other == PAYLOAD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PayloadPropertyIri> for &str {
	fn eq(&self, other: &PayloadPropertyIri) -> bool {
		*self == PAYLOAD_PROPERTY_IRI_HTTP || *self == PAYLOAD_PROPERTY_IRI_HTTPS
	}
}
pub struct PayloadPropertyIriOrLabel;
impl PartialEq<&str> for PayloadPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PayloadPropertyIri || *other == PAYLOAD_PROPERTY_LABEL
	}
}
impl PartialEq<PayloadPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PayloadPropertyIriOrLabel) -> bool {
		*self == PayloadPropertyIri || *self == PAYLOAD_PROPERTY_LABEL
	}
}
