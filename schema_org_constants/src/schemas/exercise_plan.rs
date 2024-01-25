/// <https://schema.org/ExercisePlan>
pub const EXERCISE_PLAN_IRI_HTTP: &str = "http://schema.org/ExercisePlan";
/// <https://schema.org/ExercisePlan>
pub const EXERCISE_PLAN_IRI_HTTPS: &str = "https://schema.org/ExercisePlan";
/// <https://schema.org/ExercisePlan>
pub const EXERCISE_PLAN_LABEL: &str = "ExercisePlan";
pub struct ExercisePlanIri;
impl PartialEq<&str> for ExercisePlanIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXERCISE_PLAN_IRI_HTTP || *other == EXERCISE_PLAN_IRI_HTTPS
	}
}
impl PartialEq<ExercisePlanIri> for &str {
	fn eq(&self, other: &ExercisePlanIri) -> bool {
		*self == EXERCISE_PLAN_IRI_HTTP || *self == EXERCISE_PLAN_IRI_HTTPS
	}
}
pub struct ExercisePlanIriOrLabel;
impl PartialEq<&str> for ExercisePlanIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExercisePlanIri || *other == EXERCISE_PLAN_LABEL
	}
}
impl PartialEq<ExercisePlanIriOrLabel> for &str {
	fn eq(&self, other: &ExercisePlanIriOrLabel) -> bool {
		*self == ExercisePlanIri || *self == EXERCISE_PLAN_LABEL
	}
}
