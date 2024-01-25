/// <https://schema.org/CompositeWithTrainedAlgorithmicMediaDigitalSource>
pub const COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/CompositeWithTrainedAlgorithmicMediaDigitalSource";
/// <https://schema.org/CompositeWithTrainedAlgorithmicMediaDigitalSource>
pub const COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/CompositeWithTrainedAlgorithmicMediaDigitalSource";
/// <https://schema.org/CompositeWithTrainedAlgorithmicMediaDigitalSource>
pub const COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL: &str =
	"CompositeWithTrainedAlgorithmicMediaDigitalSource";
pub struct CompositeWithTrainedAlgorithmicMediaDigitalSourceIri;
impl PartialEq<&str> for CompositeWithTrainedAlgorithmicMediaDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP
			|| *other == COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<CompositeWithTrainedAlgorithmicMediaDigitalSourceIri> for &str {
	fn eq(&self, other: &CompositeWithTrainedAlgorithmicMediaDigitalSourceIri) -> bool {
		*self == COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP
			|| *self == COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct CompositeWithTrainedAlgorithmicMediaDigitalSourceIriOrLabel;
impl PartialEq<&str> for CompositeWithTrainedAlgorithmicMediaDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompositeWithTrainedAlgorithmicMediaDigitalSourceIri
			|| *other == COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<CompositeWithTrainedAlgorithmicMediaDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &CompositeWithTrainedAlgorithmicMediaDigitalSourceIriOrLabel) -> bool {
		*self == CompositeWithTrainedAlgorithmicMediaDigitalSourceIri
			|| *self == COMPOSITE_WITH_TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL
	}
}
