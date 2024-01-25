/// <https://schema.org/MaximumDoseSchedule>
pub const MAXIMUM_DOSE_SCHEDULE_IRI_HTTP: &str = "http://schema.org/MaximumDoseSchedule";
/// <https://schema.org/MaximumDoseSchedule>
pub const MAXIMUM_DOSE_SCHEDULE_IRI_HTTPS: &str = "https://schema.org/MaximumDoseSchedule";
/// <https://schema.org/MaximumDoseSchedule>
pub const MAXIMUM_DOSE_SCHEDULE_LABEL: &str = "MaximumDoseSchedule";
pub struct MaximumDoseScheduleIri;
impl PartialEq<&str> for MaximumDoseScheduleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAXIMUM_DOSE_SCHEDULE_IRI_HTTP || *other == MAXIMUM_DOSE_SCHEDULE_IRI_HTTPS
	}
}
impl PartialEq<MaximumDoseScheduleIri> for &str {
	fn eq(&self, other: &MaximumDoseScheduleIri) -> bool {
		*self == MAXIMUM_DOSE_SCHEDULE_IRI_HTTP || *self == MAXIMUM_DOSE_SCHEDULE_IRI_HTTPS
	}
}
pub struct MaximumDoseScheduleIriOrLabel;
impl PartialEq<&str> for MaximumDoseScheduleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaximumDoseScheduleIri || *other == MAXIMUM_DOSE_SCHEDULE_LABEL
	}
}
impl PartialEq<MaximumDoseScheduleIriOrLabel> for &str {
	fn eq(&self, other: &MaximumDoseScheduleIriOrLabel) -> bool {
		*self == MaximumDoseScheduleIri || *self == MAXIMUM_DOSE_SCHEDULE_LABEL
	}
}
