/// <https://schema.org/exercisePlan>
pub const EXERCISE_PLAN_PROPERTY_IRI_HTTP: &str = "http://schema.org/exercisePlan";
/// <https://schema.org/exercisePlan>
pub const EXERCISE_PLAN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/exercisePlan";
/// <https://schema.org/exercisePlan>
pub const EXERCISE_PLAN_PROPERTY_LABEL: &str = "exercisePlan";
pub struct ExercisePlanPropertyIri;
impl PartialEq<&str> for ExercisePlanPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXERCISE_PLAN_PROPERTY_IRI_HTTP || *other == EXERCISE_PLAN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExercisePlanPropertyIri> for &str {
	fn eq(&self, other: &ExercisePlanPropertyIri) -> bool {
		*self == EXERCISE_PLAN_PROPERTY_IRI_HTTP || *self == EXERCISE_PLAN_PROPERTY_IRI_HTTPS
	}
}
pub struct ExercisePlanPropertyIriOrLabel;
impl PartialEq<&str> for ExercisePlanPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExercisePlanPropertyIri || *other == EXERCISE_PLAN_PROPERTY_LABEL
	}
}
impl PartialEq<ExercisePlanPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExercisePlanPropertyIriOrLabel) -> bool {
		*self == ExercisePlanPropertyIri || *self == EXERCISE_PLAN_PROPERTY_LABEL
	}
}
