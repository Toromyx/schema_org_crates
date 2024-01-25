/// <https://schema.org/DoseSchedule>
pub const DOSE_SCHEDULE_IRI_HTTP: &str = "http://schema.org/DoseSchedule";
/// <https://schema.org/DoseSchedule>
pub const DOSE_SCHEDULE_IRI_HTTPS: &str = "https://schema.org/DoseSchedule";
/// <https://schema.org/DoseSchedule>
pub const DOSE_SCHEDULE_LABEL: &str = "DoseSchedule";
pub struct DoseScheduleIri;
impl PartialEq<&str> for DoseScheduleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOSE_SCHEDULE_IRI_HTTP || *other == DOSE_SCHEDULE_IRI_HTTPS
	}
}
impl PartialEq<DoseScheduleIri> for &str {
	fn eq(&self, other: &DoseScheduleIri) -> bool {
		*self == DOSE_SCHEDULE_IRI_HTTP || *self == DOSE_SCHEDULE_IRI_HTTPS
	}
}
pub struct DoseScheduleIriOrLabel;
impl PartialEq<&str> for DoseScheduleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DoseScheduleIri || *other == DOSE_SCHEDULE_LABEL
	}
}
impl PartialEq<DoseScheduleIriOrLabel> for &str {
	fn eq(&self, other: &DoseScheduleIriOrLabel) -> bool {
		*self == DoseScheduleIri || *self == DOSE_SCHEDULE_LABEL
	}
}
