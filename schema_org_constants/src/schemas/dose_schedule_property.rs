/// <https://schema.org/doseSchedule>
pub const DOSE_SCHEDULE_PROPERTY_IRI_HTTP: &str = "http://schema.org/doseSchedule";
/// <https://schema.org/doseSchedule>
pub const DOSE_SCHEDULE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/doseSchedule";
/// <https://schema.org/doseSchedule>
pub const DOSE_SCHEDULE_PROPERTY_LABEL: &str = "doseSchedule";
pub struct DoseSchedulePropertyIri;
impl PartialEq<&str> for DoseSchedulePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOSE_SCHEDULE_PROPERTY_IRI_HTTP || *other == DOSE_SCHEDULE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DoseSchedulePropertyIri> for &str {
	fn eq(&self, other: &DoseSchedulePropertyIri) -> bool {
		*self == DOSE_SCHEDULE_PROPERTY_IRI_HTTP || *self == DOSE_SCHEDULE_PROPERTY_IRI_HTTPS
	}
}
pub struct DoseSchedulePropertyIriOrLabel;
impl PartialEq<&str> for DoseSchedulePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DoseSchedulePropertyIri || *other == DOSE_SCHEDULE_PROPERTY_LABEL
	}
}
impl PartialEq<DoseSchedulePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DoseSchedulePropertyIriOrLabel) -> bool {
		*self == DoseSchedulePropertyIri || *self == DOSE_SCHEDULE_PROPERTY_LABEL
	}
}
