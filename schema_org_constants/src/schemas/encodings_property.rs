/// <https://schema.org/encodings>
#[deprecated = "This schema is superseded by <https://schema.org/encoding>."]
pub const ENCODINGS_PROPERTY_IRI_HTTP: &str = "http://schema.org/encodings";
/// <https://schema.org/encodings>
#[deprecated = "This schema is superseded by <https://schema.org/encoding>."]
pub const ENCODINGS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/encodings";
/// <https://schema.org/encodings>
#[deprecated = "This schema is superseded by <https://schema.org/encoding>."]
pub const ENCODINGS_PROPERTY_LABEL: &str = "encodings";
pub struct EncodingsPropertyIri;
impl PartialEq<&str> for EncodingsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENCODINGS_PROPERTY_IRI_HTTP || *other == ENCODINGS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EncodingsPropertyIri> for &str {
	fn eq(&self, other: &EncodingsPropertyIri) -> bool {
		*self == ENCODINGS_PROPERTY_IRI_HTTP || *self == ENCODINGS_PROPERTY_IRI_HTTPS
	}
}
pub struct EncodingsPropertyIriOrLabel;
impl PartialEq<&str> for EncodingsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EncodingsPropertyIri || *other == ENCODINGS_PROPERTY_LABEL
	}
}
impl PartialEq<EncodingsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EncodingsPropertyIriOrLabel) -> bool {
		*self == EncodingsPropertyIri || *self == ENCODINGS_PROPERTY_LABEL
	}
}
