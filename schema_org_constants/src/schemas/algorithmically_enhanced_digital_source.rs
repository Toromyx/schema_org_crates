/// <https://schema.org/AlgorithmicallyEnhancedDigitalSource>
pub const ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/AlgorithmicallyEnhancedDigitalSource";
/// <https://schema.org/AlgorithmicallyEnhancedDigitalSource>
pub const ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/AlgorithmicallyEnhancedDigitalSource";
/// <https://schema.org/AlgorithmicallyEnhancedDigitalSource>
pub const ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_LABEL: &str =
	"AlgorithmicallyEnhancedDigitalSource";
pub struct AlgorithmicallyEnhancedDigitalSourceIri;
impl PartialEq<&str> for AlgorithmicallyEnhancedDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_IRI_HTTP
			|| *other == ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<AlgorithmicallyEnhancedDigitalSourceIri> for &str {
	fn eq(&self, other: &AlgorithmicallyEnhancedDigitalSourceIri) -> bool {
		*self == ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_IRI_HTTP
			|| *self == ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct AlgorithmicallyEnhancedDigitalSourceIriOrLabel;
impl PartialEq<&str> for AlgorithmicallyEnhancedDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlgorithmicallyEnhancedDigitalSourceIri
			|| *other == ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<AlgorithmicallyEnhancedDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &AlgorithmicallyEnhancedDigitalSourceIriOrLabel) -> bool {
		*self == AlgorithmicallyEnhancedDigitalSourceIri
			|| *self == ALGORITHMICALLY_ENHANCED_DIGITAL_SOURCE_LABEL
	}
}
