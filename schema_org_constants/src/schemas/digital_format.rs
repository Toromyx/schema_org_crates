/// <https://schema.org/DigitalFormat>
pub const DIGITAL_FORMAT_IRI_HTTP: &str = "http://schema.org/DigitalFormat";
/// <https://schema.org/DigitalFormat>
pub const DIGITAL_FORMAT_IRI_HTTPS: &str = "https://schema.org/DigitalFormat";
/// <https://schema.org/DigitalFormat>
pub const DIGITAL_FORMAT_LABEL: &str = "DigitalFormat";
pub struct DigitalFormatIri;
impl PartialEq<&str> for DigitalFormatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_FORMAT_IRI_HTTP || *other == DIGITAL_FORMAT_IRI_HTTPS
	}
}
impl PartialEq<DigitalFormatIri> for &str {
	fn eq(&self, other: &DigitalFormatIri) -> bool {
		*self == DIGITAL_FORMAT_IRI_HTTP || *self == DIGITAL_FORMAT_IRI_HTTPS
	}
}
pub struct DigitalFormatIriOrLabel;
impl PartialEq<&str> for DigitalFormatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalFormatIri || *other == DIGITAL_FORMAT_LABEL
	}
}
impl PartialEq<DigitalFormatIriOrLabel> for &str {
	fn eq(&self, other: &DigitalFormatIriOrLabel) -> bool {
		*self == DigitalFormatIri || *self == DIGITAL_FORMAT_LABEL
	}
}
