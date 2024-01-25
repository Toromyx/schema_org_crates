/// <https://schema.org/ExerciseGym>
pub const EXERCISE_GYM_IRI_HTTP: &str = "http://schema.org/ExerciseGym";
/// <https://schema.org/ExerciseGym>
pub const EXERCISE_GYM_IRI_HTTPS: &str = "https://schema.org/ExerciseGym";
/// <https://schema.org/ExerciseGym>
pub const EXERCISE_GYM_LABEL: &str = "ExerciseGym";
pub struct ExerciseGymIri;
impl PartialEq<&str> for ExerciseGymIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXERCISE_GYM_IRI_HTTP || *other == EXERCISE_GYM_IRI_HTTPS
	}
}
impl PartialEq<ExerciseGymIri> for &str {
	fn eq(&self, other: &ExerciseGymIri) -> bool {
		*self == EXERCISE_GYM_IRI_HTTP || *self == EXERCISE_GYM_IRI_HTTPS
	}
}
pub struct ExerciseGymIriOrLabel;
impl PartialEq<&str> for ExerciseGymIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExerciseGymIri || *other == EXERCISE_GYM_LABEL
	}
}
impl PartialEq<ExerciseGymIriOrLabel> for &str {
	fn eq(&self, other: &ExerciseGymIriOrLabel) -> bool {
		*self == ExerciseGymIri || *self == EXERCISE_GYM_LABEL
	}
}
