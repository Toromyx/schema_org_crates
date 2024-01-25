/// <https://schema.org/ExerciseAction>
pub const EXERCISE_ACTION_IRI_HTTP: &str = "http://schema.org/ExerciseAction";
/// <https://schema.org/ExerciseAction>
pub const EXERCISE_ACTION_IRI_HTTPS: &str = "https://schema.org/ExerciseAction";
/// <https://schema.org/ExerciseAction>
pub const EXERCISE_ACTION_LABEL: &str = "ExerciseAction";
pub struct ExerciseActionIri;
impl PartialEq<&str> for ExerciseActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXERCISE_ACTION_IRI_HTTP || *other == EXERCISE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ExerciseActionIri> for &str {
	fn eq(&self, other: &ExerciseActionIri) -> bool {
		*self == EXERCISE_ACTION_IRI_HTTP || *self == EXERCISE_ACTION_IRI_HTTPS
	}
}
pub struct ExerciseActionIriOrLabel;
impl PartialEq<&str> for ExerciseActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExerciseActionIri || *other == EXERCISE_ACTION_LABEL
	}
}
impl PartialEq<ExerciseActionIriOrLabel> for &str {
	fn eq(&self, other: &ExerciseActionIriOrLabel) -> bool {
		*self == ExerciseActionIri || *self == EXERCISE_ACTION_LABEL
	}
}
