/// <https://schema.org/byMonthDay>
pub const BY_MONTH_DAY_PROPERTY_IRI_HTTP: &str = "http://schema.org/byMonthDay";
/// <https://schema.org/byMonthDay>
pub const BY_MONTH_DAY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/byMonthDay";
/// <https://schema.org/byMonthDay>
pub const BY_MONTH_DAY_PROPERTY_LABEL: &str = "byMonthDay";
pub struct ByMonthDayPropertyIri;
impl PartialEq<&str> for ByMonthDayPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BY_MONTH_DAY_PROPERTY_IRI_HTTP || *other == BY_MONTH_DAY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ByMonthDayPropertyIri> for &str {
	fn eq(&self, other: &ByMonthDayPropertyIri) -> bool {
		*self == BY_MONTH_DAY_PROPERTY_IRI_HTTP || *self == BY_MONTH_DAY_PROPERTY_IRI_HTTPS
	}
}
pub struct ByMonthDayPropertyIriOrLabel;
impl PartialEq<&str> for ByMonthDayPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ByMonthDayPropertyIri || *other == BY_MONTH_DAY_PROPERTY_LABEL
	}
}
impl PartialEq<ByMonthDayPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ByMonthDayPropertyIriOrLabel) -> bool {
		*self == ByMonthDayPropertyIri || *self == BY_MONTH_DAY_PROPERTY_LABEL
	}
}
