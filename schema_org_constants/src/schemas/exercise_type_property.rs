/// <https://schema.org/exerciseType>
pub const EXERCISE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/exerciseType";
/// <https://schema.org/exerciseType>
pub const EXERCISE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/exerciseType";
/// <https://schema.org/exerciseType>
pub const EXERCISE_TYPE_PROPERTY_LABEL: &str = "exerciseType";
pub struct ExerciseTypePropertyIri;
impl PartialEq<&str> for ExerciseTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXERCISE_TYPE_PROPERTY_IRI_HTTP || *other == EXERCISE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExerciseTypePropertyIri> for &str {
	fn eq(&self, other: &ExerciseTypePropertyIri) -> bool {
		*self == EXERCISE_TYPE_PROPERTY_IRI_HTTP || *self == EXERCISE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct ExerciseTypePropertyIriOrLabel;
impl PartialEq<&str> for ExerciseTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExerciseTypePropertyIri || *other == EXERCISE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<ExerciseTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExerciseTypePropertyIriOrLabel) -> bool {
		*self == ExerciseTypePropertyIri || *self == EXERCISE_TYPE_PROPERTY_LABEL
	}
}
