/// <https://schema.org/encodingType>
pub const ENCODING_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/encodingType";
/// <https://schema.org/encodingType>
pub const ENCODING_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/encodingType";
/// <https://schema.org/encodingType>
pub const ENCODING_TYPE_PROPERTY_LABEL: &str = "encodingType";
pub struct EncodingTypePropertyIri;
impl PartialEq<&str> for EncodingTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENCODING_TYPE_PROPERTY_IRI_HTTP || *other == ENCODING_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EncodingTypePropertyIri> for &str {
	fn eq(&self, other: &EncodingTypePropertyIri) -> bool {
		*self == ENCODING_TYPE_PROPERTY_IRI_HTTP || *self == ENCODING_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct EncodingTypePropertyIriOrLabel;
impl PartialEq<&str> for EncodingTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EncodingTypePropertyIri || *other == ENCODING_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<EncodingTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EncodingTypePropertyIriOrLabel) -> bool {
		*self == EncodingTypePropertyIri || *self == ENCODING_TYPE_PROPERTY_LABEL
	}
}
