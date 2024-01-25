/// <https://schema.org/StrengthTraining>
pub const STRENGTH_TRAINING_IRI_HTTP: &str = "http://schema.org/StrengthTraining";
/// <https://schema.org/StrengthTraining>
pub const STRENGTH_TRAINING_IRI_HTTPS: &str = "https://schema.org/StrengthTraining";
/// <https://schema.org/StrengthTraining>
pub const STRENGTH_TRAINING_LABEL: &str = "StrengthTraining";
pub struct StrengthTrainingIri;
impl PartialEq<&str> for StrengthTrainingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STRENGTH_TRAINING_IRI_HTTP || *other == STRENGTH_TRAINING_IRI_HTTPS
	}
}
impl PartialEq<StrengthTrainingIri> for &str {
	fn eq(&self, other: &StrengthTrainingIri) -> bool {
		*self == STRENGTH_TRAINING_IRI_HTTP || *self == STRENGTH_TRAINING_IRI_HTTPS
	}
}
pub struct StrengthTrainingIriOrLabel;
impl PartialEq<&str> for StrengthTrainingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StrengthTrainingIri || *other == STRENGTH_TRAINING_LABEL
	}
}
impl PartialEq<StrengthTrainingIriOrLabel> for &str {
	fn eq(&self, other: &StrengthTrainingIriOrLabel) -> bool {
		*self == StrengthTrainingIri || *self == STRENGTH_TRAINING_LABEL
	}
}
