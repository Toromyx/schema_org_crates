/// <https://schema.org/reportNumber>
pub const REPORT_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/reportNumber";
/// <https://schema.org/reportNumber>
pub const REPORT_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/reportNumber";
/// <https://schema.org/reportNumber>
pub const REPORT_NUMBER_PROPERTY_LABEL: &str = "reportNumber";
pub struct ReportNumberPropertyIri;
impl PartialEq<&str> for ReportNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPORT_NUMBER_PROPERTY_IRI_HTTP || *other == REPORT_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReportNumberPropertyIri> for &str {
	fn eq(&self, other: &ReportNumberPropertyIri) -> bool {
		*self == REPORT_NUMBER_PROPERTY_IRI_HTTP || *self == REPORT_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct ReportNumberPropertyIriOrLabel;
impl PartialEq<&str> for ReportNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReportNumberPropertyIri || *other == REPORT_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<ReportNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReportNumberPropertyIriOrLabel) -> bool {
		*self == ReportNumberPropertyIri || *self == REPORT_NUMBER_PROPERTY_LABEL
	}
}
