/// <https://schema.org/exerciseRelatedDiet>
pub const EXERCISE_RELATED_DIET_PROPERTY_IRI_HTTP: &str = "http://schema.org/exerciseRelatedDiet";
/// <https://schema.org/exerciseRelatedDiet>
pub const EXERCISE_RELATED_DIET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/exerciseRelatedDiet";
/// <https://schema.org/exerciseRelatedDiet>
pub const EXERCISE_RELATED_DIET_PROPERTY_LABEL: &str = "exerciseRelatedDiet";
pub struct ExerciseRelatedDietPropertyIri;
impl PartialEq<&str> for ExerciseRelatedDietPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXERCISE_RELATED_DIET_PROPERTY_IRI_HTTP
			|| *other == EXERCISE_RELATED_DIET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExerciseRelatedDietPropertyIri> for &str {
	fn eq(&self, other: &ExerciseRelatedDietPropertyIri) -> bool {
		*self == EXERCISE_RELATED_DIET_PROPERTY_IRI_HTTP
			|| *self == EXERCISE_RELATED_DIET_PROPERTY_IRI_HTTPS
	}
}
pub struct ExerciseRelatedDietPropertyIriOrLabel;
impl PartialEq<&str> for ExerciseRelatedDietPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExerciseRelatedDietPropertyIri || *other == EXERCISE_RELATED_DIET_PROPERTY_LABEL
	}
}
impl PartialEq<ExerciseRelatedDietPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExerciseRelatedDietPropertyIriOrLabel) -> bool {
		*self == ExerciseRelatedDietPropertyIri || *self == EXERCISE_RELATED_DIET_PROPERTY_LABEL
	}
}
