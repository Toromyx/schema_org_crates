/// <https://schema.org/DayOfWeek>
pub const DAY_OF_WEEK_IRI_HTTP: &str = "http://schema.org/DayOfWeek";
/// <https://schema.org/DayOfWeek>
pub const DAY_OF_WEEK_IRI_HTTPS: &str = "https://schema.org/DayOfWeek";
/// <https://schema.org/DayOfWeek>
pub const DAY_OF_WEEK_LABEL: &str = "DayOfWeek";
pub struct DayOfWeekIri;
impl PartialEq<&str> for DayOfWeekIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DAY_OF_WEEK_IRI_HTTP || *other == DAY_OF_WEEK_IRI_HTTPS
	}
}
impl PartialEq<DayOfWeekIri> for &str {
	fn eq(&self, other: &DayOfWeekIri) -> bool {
		*self == DAY_OF_WEEK_IRI_HTTP || *self == DAY_OF_WEEK_IRI_HTTPS
	}
}
pub struct DayOfWeekIriOrLabel;
impl PartialEq<&str> for DayOfWeekIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DayOfWeekIri || *other == DAY_OF_WEEK_LABEL
	}
}
impl PartialEq<DayOfWeekIriOrLabel> for &str {
	fn eq(&self, other: &DayOfWeekIriOrLabel) -> bool {
		*self == DayOfWeekIri || *self == DAY_OF_WEEK_LABEL
	}
}
