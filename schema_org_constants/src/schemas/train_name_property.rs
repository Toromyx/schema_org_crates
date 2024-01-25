/// <https://schema.org/trainName>
pub const TRAIN_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/trainName";
/// <https://schema.org/trainName>
pub const TRAIN_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/trainName";
/// <https://schema.org/trainName>
pub const TRAIN_NAME_PROPERTY_LABEL: &str = "trainName";
pub struct TrainNamePropertyIri;
impl PartialEq<&str> for TrainNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAIN_NAME_PROPERTY_IRI_HTTP || *other == TRAIN_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrainNamePropertyIri> for &str {
	fn eq(&self, other: &TrainNamePropertyIri) -> bool {
		*self == TRAIN_NAME_PROPERTY_IRI_HTTP || *self == TRAIN_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct TrainNamePropertyIriOrLabel;
impl PartialEq<&str> for TrainNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrainNamePropertyIri || *other == TRAIN_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<TrainNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrainNamePropertyIriOrLabel) -> bool {
		*self == TrainNamePropertyIri || *self == TRAIN_NAME_PROPERTY_LABEL
	}
}
