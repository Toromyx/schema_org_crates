/// <https://schema.org/dayOfWeek>
pub const DAY_OF_WEEK_PROPERTY_IRI_HTTP: &str = "http://schema.org/dayOfWeek";
/// <https://schema.org/dayOfWeek>
pub const DAY_OF_WEEK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dayOfWeek";
/// <https://schema.org/dayOfWeek>
pub const DAY_OF_WEEK_PROPERTY_LABEL: &str = "dayOfWeek";
pub struct DayOfWeekPropertyIri;
impl PartialEq<&str> for DayOfWeekPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DAY_OF_WEEK_PROPERTY_IRI_HTTP || *other == DAY_OF_WEEK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DayOfWeekPropertyIri> for &str {
	fn eq(&self, other: &DayOfWeekPropertyIri) -> bool {
		*self == DAY_OF_WEEK_PROPERTY_IRI_HTTP || *self == DAY_OF_WEEK_PROPERTY_IRI_HTTPS
	}
}
pub struct DayOfWeekPropertyIriOrLabel;
impl PartialEq<&str> for DayOfWeekPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DayOfWeekPropertyIri || *other == DAY_OF_WEEK_PROPERTY_LABEL
	}
}
impl PartialEq<DayOfWeekPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DayOfWeekPropertyIriOrLabel) -> bool {
		*self == DayOfWeekPropertyIri || *self == DAY_OF_WEEK_PROPERTY_LABEL
	}
}
