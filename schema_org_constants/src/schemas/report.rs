/// <https://schema.org/Report>
pub const REPORT_IRI_HTTP: &str = "http://schema.org/Report";
/// <https://schema.org/Report>
pub const REPORT_IRI_HTTPS: &str = "https://schema.org/Report";
/// <https://schema.org/Report>
pub const REPORT_LABEL: &str = "Report";
pub struct ReportIri;
impl PartialEq<&str> for ReportIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REPORT_IRI_HTTP || *other == REPORT_IRI_HTTPS
	}
}
impl PartialEq<ReportIri> for &str {
	fn eq(&self, other: &ReportIri) -> bool {
		*self == REPORT_IRI_HTTP || *self == REPORT_IRI_HTTPS
	}
}
pub struct ReportIriOrLabel;
impl PartialEq<&str> for ReportIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReportIri || *other == REPORT_LABEL
	}
}
impl PartialEq<ReportIriOrLabel> for &str {
	fn eq(&self, other: &ReportIriOrLabel) -> bool {
		*self == ReportIri || *self == REPORT_LABEL
	}
}
