/// <https://schema.org/DigitalArtDigitalSource>
pub const DIGITAL_ART_DIGITAL_SOURCE_IRI_HTTP: &str = "http://schema.org/DigitalArtDigitalSource";
/// <https://schema.org/DigitalArtDigitalSource>
pub const DIGITAL_ART_DIGITAL_SOURCE_IRI_HTTPS: &str = "https://schema.org/DigitalArtDigitalSource";
/// <https://schema.org/DigitalArtDigitalSource>
pub const DIGITAL_ART_DIGITAL_SOURCE_LABEL: &str = "DigitalArtDigitalSource";
pub struct DigitalArtDigitalSourceIri;
impl PartialEq<&str> for DigitalArtDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_ART_DIGITAL_SOURCE_IRI_HTTP
			|| *other == DIGITAL_ART_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<DigitalArtDigitalSourceIri> for &str {
	fn eq(&self, other: &DigitalArtDigitalSourceIri) -> bool {
		*self == DIGITAL_ART_DIGITAL_SOURCE_IRI_HTTP
			|| *self == DIGITAL_ART_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct DigitalArtDigitalSourceIriOrLabel;
impl PartialEq<&str> for DigitalArtDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalArtDigitalSourceIri || *other == DIGITAL_ART_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<DigitalArtDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &DigitalArtDigitalSourceIriOrLabel) -> bool {
		*self == DigitalArtDigitalSourceIri || *self == DIGITAL_ART_DIGITAL_SOURCE_LABEL
	}
}
