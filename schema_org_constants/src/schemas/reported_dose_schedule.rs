/// <https://schema.org/ReportedDoseSchedule>
pub const REPORTED_DOSE_SCHEDULE_IRI_HTTP: &str = "http://schema.org/ReportedDoseSchedule";
/// <https://schema.org/ReportedDoseSchedule>
pub const REPORTED_DOSE_SCHEDULE_IRI_HTTPS: &str = "https://schema.org/ReportedDoseSchedule";
/// <https://schema.org/ReportedDoseSchedule>
pub const REPORTED_DOSE_SCHEDULE_LABEL: &str = "ReportedDoseSchedule";
pub struct ReportedDoseScheduleIri;
impl PartialEq<&str> for ReportedDoseScheduleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPORTED_DOSE_SCHEDULE_IRI_HTTP || *other == REPORTED_DOSE_SCHEDULE_IRI_HTTPS
	}
}
impl PartialEq<ReportedDoseScheduleIri> for &str {
	fn eq(&self, other: &ReportedDoseScheduleIri) -> bool {
		*self == REPORTED_DOSE_SCHEDULE_IRI_HTTP || *self == REPORTED_DOSE_SCHEDULE_IRI_HTTPS
	}
}
pub struct ReportedDoseScheduleIriOrLabel;
impl PartialEq<&str> for ReportedDoseScheduleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReportedDoseScheduleIri || *other == REPORTED_DOSE_SCHEDULE_LABEL
	}
}
impl PartialEq<ReportedDoseScheduleIriOrLabel> for &str {
	fn eq(&self, other: &ReportedDoseScheduleIriOrLabel) -> bool {
		*self == ReportedDoseScheduleIri || *self == REPORTED_DOSE_SCHEDULE_LABEL
	}
}
