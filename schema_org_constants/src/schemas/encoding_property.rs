/// <https://schema.org/encoding>
pub const ENCODING_PROPERTY_IRI_HTTP: &str = "http://schema.org/encoding";
/// <https://schema.org/encoding>
pub const ENCODING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/encoding";
/// <https://schema.org/encoding>
pub const ENCODING_PROPERTY_LABEL: &str = "encoding";
pub struct EncodingPropertyIri;
impl PartialEq<&str> for EncodingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENCODING_PROPERTY_IRI_HTTP || *other == ENCODING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EncodingPropertyIri> for &str {
	fn eq(&self, other: &EncodingPropertyIri) -> bool {
		*self == ENCODING_PROPERTY_IRI_HTTP || *self == ENCODING_PROPERTY_IRI_HTTPS
	}
}
pub struct EncodingPropertyIriOrLabel;
impl PartialEq<&str> for EncodingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EncodingPropertyIri || *other == ENCODING_PROPERTY_LABEL
	}
}
impl PartialEq<EncodingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EncodingPropertyIriOrLabel) -> bool {
		*self == EncodingPropertyIri || *self == ENCODING_PROPERTY_LABEL
	}
}
