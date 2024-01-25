/// <https://schema.org/byMonth>
pub const BY_MONTH_PROPERTY_IRI_HTTP: &str = "http://schema.org/byMonth";
/// <https://schema.org/byMonth>
pub const BY_MONTH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/byMonth";
/// <https://schema.org/byMonth>
pub const BY_MONTH_PROPERTY_LABEL: &str = "byMonth";
pub struct ByMonthPropertyIri;
impl PartialEq<&str> for ByMonthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BY_MONTH_PROPERTY_IRI_HTTP || *other == BY_MONTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ByMonthPropertyIri> for &str {
	fn eq(&self, other: &ByMonthPropertyIri) -> bool {
		*self == BY_MONTH_PROPERTY_IRI_HTTP || *self == BY_MONTH_PROPERTY_IRI_HTTPS
	}
}
pub struct ByMonthPropertyIriOrLabel;
impl PartialEq<&str> for ByMonthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ByMonthPropertyIri || *other == BY_MONTH_PROPERTY_LABEL
	}
}
impl PartialEq<ByMonthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ByMonthPropertyIriOrLabel) -> bool {
		*self == ByMonthPropertyIri || *self == BY_MONTH_PROPERTY_LABEL
	}
}
