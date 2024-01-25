/// <https://schema.org/AlgorithmicMediaDigitalSource>
pub const ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/AlgorithmicMediaDigitalSource";
/// <https://schema.org/AlgorithmicMediaDigitalSource>
pub const ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/AlgorithmicMediaDigitalSource";
/// <https://schema.org/AlgorithmicMediaDigitalSource>
pub const ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL: &str = "AlgorithmicMediaDigitalSource";
pub struct AlgorithmicMediaDigitalSourceIri;
impl PartialEq<&str> for AlgorithmicMediaDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP
			|| *other == ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<AlgorithmicMediaDigitalSourceIri> for &str {
	fn eq(&self, other: &AlgorithmicMediaDigitalSourceIri) -> bool {
		*self == ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP
			|| *self == ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct AlgorithmicMediaDigitalSourceIriOrLabel;
impl PartialEq<&str> for AlgorithmicMediaDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlgorithmicMediaDigitalSourceIri
			|| *other == ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<AlgorithmicMediaDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &AlgorithmicMediaDigitalSourceIriOrLabel) -> bool {
		*self == AlgorithmicMediaDigitalSourceIri || *self == ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL
	}
}
