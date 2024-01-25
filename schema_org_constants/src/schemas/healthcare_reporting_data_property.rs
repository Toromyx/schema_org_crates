/// <https://schema.org/healthcareReportingData>
pub const HEALTHCARE_REPORTING_DATA_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/healthcareReportingData";
/// <https://schema.org/healthcareReportingData>
pub const HEALTHCARE_REPORTING_DATA_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/healthcareReportingData";
/// <https://schema.org/healthcareReportingData>
pub const HEALTHCARE_REPORTING_DATA_PROPERTY_LABEL: &str = "healthcareReportingData";
pub struct HealthcareReportingDataPropertyIri;
impl PartialEq<&str> for HealthcareReportingDataPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTHCARE_REPORTING_DATA_PROPERTY_IRI_HTTP
			|| *other == HEALTHCARE_REPORTING_DATA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthcareReportingDataPropertyIri> for &str {
	fn eq(&self, other: &HealthcareReportingDataPropertyIri) -> bool {
		*self == HEALTHCARE_REPORTING_DATA_PROPERTY_IRI_HTTP
			|| *self == HEALTHCARE_REPORTING_DATA_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthcareReportingDataPropertyIriOrLabel;
impl PartialEq<&str> for HealthcareReportingDataPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthcareReportingDataPropertyIri
			|| *other == HEALTHCARE_REPORTING_DATA_PROPERTY_LABEL
	}
}
impl PartialEq<HealthcareReportingDataPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthcareReportingDataPropertyIriOrLabel) -> bool {
		*self == HealthcareReportingDataPropertyIri
			|| *self == HEALTHCARE_REPORTING_DATA_PROPERTY_LABEL
	}
}
