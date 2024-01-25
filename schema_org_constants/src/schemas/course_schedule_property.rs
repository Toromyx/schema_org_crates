/// <https://schema.org/courseSchedule>
pub const COURSE_SCHEDULE_PROPERTY_IRI_HTTP: &str = "http://schema.org/courseSchedule";
/// <https://schema.org/courseSchedule>
pub const COURSE_SCHEDULE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/courseSchedule";
/// <https://schema.org/courseSchedule>
pub const COURSE_SCHEDULE_PROPERTY_LABEL: &str = "courseSchedule";
pub struct CourseSchedulePropertyIri;
impl PartialEq<&str> for CourseSchedulePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COURSE_SCHEDULE_PROPERTY_IRI_HTTP || *other == COURSE_SCHEDULE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CourseSchedulePropertyIri> for &str {
	fn eq(&self, other: &CourseSchedulePropertyIri) -> bool {
		*self == COURSE_SCHEDULE_PROPERTY_IRI_HTTP || *self == COURSE_SCHEDULE_PROPERTY_IRI_HTTPS
	}
}
pub struct CourseSchedulePropertyIriOrLabel;
impl PartialEq<&str> for CourseSchedulePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CourseSchedulePropertyIri || *other == COURSE_SCHEDULE_PROPERTY_LABEL
	}
}
impl PartialEq<CourseSchedulePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CourseSchedulePropertyIriOrLabel) -> bool {
		*self == CourseSchedulePropertyIri || *self == COURSE_SCHEDULE_PROPERTY_LABEL
	}
}
