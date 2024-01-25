/// <https://schema.org/exerciseCourse>
pub const EXERCISE_COURSE_PROPERTY_IRI_HTTP: &str = "http://schema.org/exerciseCourse";
/// <https://schema.org/exerciseCourse>
pub const EXERCISE_COURSE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/exerciseCourse";
/// <https://schema.org/exerciseCourse>
pub const EXERCISE_COURSE_PROPERTY_LABEL: &str = "exerciseCourse";
pub struct ExerciseCoursePropertyIri;
impl PartialEq<&str> for ExerciseCoursePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXERCISE_COURSE_PROPERTY_IRI_HTTP || *other == EXERCISE_COURSE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExerciseCoursePropertyIri> for &str {
	fn eq(&self, other: &ExerciseCoursePropertyIri) -> bool {
		*self == EXERCISE_COURSE_PROPERTY_IRI_HTTP || *self == EXERCISE_COURSE_PROPERTY_IRI_HTTPS
	}
}
pub struct ExerciseCoursePropertyIriOrLabel;
impl PartialEq<&str> for ExerciseCoursePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExerciseCoursePropertyIri || *other == EXERCISE_COURSE_PROPERTY_LABEL
	}
}
impl PartialEq<ExerciseCoursePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExerciseCoursePropertyIriOrLabel) -> bool {
		*self == ExerciseCoursePropertyIri || *self == EXERCISE_COURSE_PROPERTY_LABEL
	}
}
