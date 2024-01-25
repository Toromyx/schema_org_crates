/// <https://schema.org/TrainedAlgorithmicMediaDigitalSource>
pub const TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/TrainedAlgorithmicMediaDigitalSource";
/// <https://schema.org/TrainedAlgorithmicMediaDigitalSource>
pub const TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/TrainedAlgorithmicMediaDigitalSource";
/// <https://schema.org/TrainedAlgorithmicMediaDigitalSource>
pub const TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL: &str =
	"TrainedAlgorithmicMediaDigitalSource";
pub struct TrainedAlgorithmicMediaDigitalSourceIri;
impl PartialEq<&str> for TrainedAlgorithmicMediaDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP
			|| *other == TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<TrainedAlgorithmicMediaDigitalSourceIri> for &str {
	fn eq(&self, other: &TrainedAlgorithmicMediaDigitalSourceIri) -> bool {
		*self == TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTP
			|| *self == TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct TrainedAlgorithmicMediaDigitalSourceIriOrLabel;
impl PartialEq<&str> for TrainedAlgorithmicMediaDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrainedAlgorithmicMediaDigitalSourceIri
			|| *other == TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<TrainedAlgorithmicMediaDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &TrainedAlgorithmicMediaDigitalSourceIriOrLabel) -> bool {
		*self == TrainedAlgorithmicMediaDigitalSourceIri
			|| *self == TRAINED_ALGORITHMIC_MEDIA_DIGITAL_SOURCE_LABEL
	}
}
