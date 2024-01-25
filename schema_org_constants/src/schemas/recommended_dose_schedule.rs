/// <https://schema.org/RecommendedDoseSchedule>
pub const RECOMMENDED_DOSE_SCHEDULE_IRI_HTTP: &str = "http://schema.org/RecommendedDoseSchedule";
/// <https://schema.org/RecommendedDoseSchedule>
pub const RECOMMENDED_DOSE_SCHEDULE_IRI_HTTPS: &str = "https://schema.org/RecommendedDoseSchedule";
/// <https://schema.org/RecommendedDoseSchedule>
pub const RECOMMENDED_DOSE_SCHEDULE_LABEL: &str = "RecommendedDoseSchedule";
pub struct RecommendedDoseScheduleIri;
impl PartialEq<&str> for RecommendedDoseScheduleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECOMMENDED_DOSE_SCHEDULE_IRI_HTTP
			|| *other == RECOMMENDED_DOSE_SCHEDULE_IRI_HTTPS
	}
}
impl PartialEq<RecommendedDoseScheduleIri> for &str {
	fn eq(&self, other: &RecommendedDoseScheduleIri) -> bool {
		*self == RECOMMENDED_DOSE_SCHEDULE_IRI_HTTP || *self == RECOMMENDED_DOSE_SCHEDULE_IRI_HTTPS
	}
}
pub struct RecommendedDoseScheduleIriOrLabel;
impl PartialEq<&str> for RecommendedDoseScheduleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecommendedDoseScheduleIri || *other == RECOMMENDED_DOSE_SCHEDULE_LABEL
	}
}
impl PartialEq<RecommendedDoseScheduleIriOrLabel> for &str {
	fn eq(&self, other: &RecommendedDoseScheduleIriOrLabel) -> bool {
		*self == RecommendedDoseScheduleIri || *self == RECOMMENDED_DOSE_SCHEDULE_LABEL
	}
}
