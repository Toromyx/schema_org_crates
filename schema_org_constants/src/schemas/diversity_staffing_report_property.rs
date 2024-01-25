/// <https://schema.org/diversityStaffingReport>
pub const DIVERSITY_STAFFING_REPORT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/diversityStaffingReport";
/// <https://schema.org/diversityStaffingReport>
pub const DIVERSITY_STAFFING_REPORT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/diversityStaffingReport";
/// <https://schema.org/diversityStaffingReport>
pub const DIVERSITY_STAFFING_REPORT_PROPERTY_LABEL: &str = "diversityStaffingReport";
pub struct DiversityStaffingReportPropertyIri;
impl PartialEq<&str> for DiversityStaffingReportPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIVERSITY_STAFFING_REPORT_PROPERTY_IRI_HTTP
			|| *other == DIVERSITY_STAFFING_REPORT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiversityStaffingReportPropertyIri> for &str {
	fn eq(&self, other: &DiversityStaffingReportPropertyIri) -> bool {
		*self == DIVERSITY_STAFFING_REPORT_PROPERTY_IRI_HTTP
			|| *self == DIVERSITY_STAFFING_REPORT_PROPERTY_IRI_HTTPS
	}
}
pub struct DiversityStaffingReportPropertyIriOrLabel;
impl PartialEq<&str> for DiversityStaffingReportPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiversityStaffingReportPropertyIri
			|| *other == DIVERSITY_STAFFING_REPORT_PROPERTY_LABEL
	}
}
impl PartialEq<DiversityStaffingReportPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiversityStaffingReportPropertyIriOrLabel) -> bool {
		*self == DiversityStaffingReportPropertyIri
			|| *self == DIVERSITY_STAFFING_REPORT_PROPERTY_LABEL
	}
}
