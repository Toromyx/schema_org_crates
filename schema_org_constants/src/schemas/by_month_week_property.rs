/// <https://schema.org/byMonthWeek>
pub const BY_MONTH_WEEK_PROPERTY_IRI_HTTP: &str = "http://schema.org/byMonthWeek";
/// <https://schema.org/byMonthWeek>
pub const BY_MONTH_WEEK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/byMonthWeek";
/// <https://schema.org/byMonthWeek>
pub const BY_MONTH_WEEK_PROPERTY_LABEL: &str = "byMonthWeek";
pub struct ByMonthWeekPropertyIri;
impl PartialEq<&str> for ByMonthWeekPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BY_MONTH_WEEK_PROPERTY_IRI_HTTP || *other == BY_MONTH_WEEK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ByMonthWeekPropertyIri> for &str {
	fn eq(&self, other: &ByMonthWeekPropertyIri) -> bool {
		*self == BY_MONTH_WEEK_PROPERTY_IRI_HTTP || *self == BY_MONTH_WEEK_PROPERTY_IRI_HTTPS
	}
}
pub struct ByMonthWeekPropertyIriOrLabel;
impl PartialEq<&str> for ByMonthWeekPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ByMonthWeekPropertyIri || *other == BY_MONTH_WEEK_PROPERTY_LABEL
	}
}
impl PartialEq<ByMonthWeekPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ByMonthWeekPropertyIriOrLabel) -> bool {
		*self == ByMonthWeekPropertyIri || *self == BY_MONTH_WEEK_PROPERTY_LABEL
	}
}
