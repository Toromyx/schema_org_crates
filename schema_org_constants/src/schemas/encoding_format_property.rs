/// <https://schema.org/encodingFormat>
pub const ENCODING_FORMAT_PROPERTY_IRI_HTTP: &str = "http://schema.org/encodingFormat";
/// <https://schema.org/encodingFormat>
pub const ENCODING_FORMAT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/encodingFormat";
/// <https://schema.org/encodingFormat>
pub const ENCODING_FORMAT_PROPERTY_LABEL: &str = "encodingFormat";
pub struct EncodingFormatPropertyIri;
impl PartialEq<&str> for EncodingFormatPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENCODING_FORMAT_PROPERTY_IRI_HTTP || *other == ENCODING_FORMAT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EncodingFormatPropertyIri> for &str {
	fn eq(&self, other: &EncodingFormatPropertyIri) -> bool {
		*self == ENCODING_FORMAT_PROPERTY_IRI_HTTP || *self == ENCODING_FORMAT_PROPERTY_IRI_HTTPS
	}
}
pub struct EncodingFormatPropertyIriOrLabel;
impl PartialEq<&str> for EncodingFormatPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EncodingFormatPropertyIri || *other == ENCODING_FORMAT_PROPERTY_LABEL
	}
}
impl PartialEq<EncodingFormatPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EncodingFormatPropertyIriOrLabel) -> bool {
		*self == EncodingFormatPropertyIri || *self == ENCODING_FORMAT_PROPERTY_LABEL
	}
}
