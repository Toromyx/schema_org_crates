/// <https://schema.org/trainNumber>
pub const TRAIN_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/trainNumber";
/// <https://schema.org/trainNumber>
pub const TRAIN_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/trainNumber";
/// <https://schema.org/trainNumber>
pub const TRAIN_NUMBER_PROPERTY_LABEL: &str = "trainNumber";
pub struct TrainNumberPropertyIri;
impl PartialEq<&str> for TrainNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAIN_NUMBER_PROPERTY_IRI_HTTP || *other == TRAIN_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrainNumberPropertyIri> for &str {
	fn eq(&self, other: &TrainNumberPropertyIri) -> bool {
		*self == TRAIN_NUMBER_PROPERTY_IRI_HTTP || *self == TRAIN_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct TrainNumberPropertyIriOrLabel;
impl PartialEq<&str> for TrainNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrainNumberPropertyIri || *other == TRAIN_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<TrainNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrainNumberPropertyIriOrLabel) -> bool {
		*self == TrainNumberPropertyIri || *self == TRAIN_NUMBER_PROPERTY_LABEL
	}
}
