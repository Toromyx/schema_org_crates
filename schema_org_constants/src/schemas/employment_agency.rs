/// <https://schema.org/EmploymentAgency>
pub const EMPLOYMENT_AGENCY_IRI_HTTP: &str = "http://schema.org/EmploymentAgency";
/// <https://schema.org/EmploymentAgency>
pub const EMPLOYMENT_AGENCY_IRI_HTTPS: &str = "https://schema.org/EmploymentAgency";
/// <https://schema.org/EmploymentAgency>
pub const EMPLOYMENT_AGENCY_LABEL: &str = "EmploymentAgency";
pub struct EmploymentAgencyIri;
impl PartialEq<&str> for EmploymentAgencyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYMENT_AGENCY_IRI_HTTP || *other == EMPLOYMENT_AGENCY_IRI_HTTPS
	}
}
impl PartialEq<EmploymentAgencyIri> for &str {
	fn eq(&self, other: &EmploymentAgencyIri) -> bool {
		*self == EMPLOYMENT_AGENCY_IRI_HTTP || *self == EMPLOYMENT_AGENCY_IRI_HTTPS
	}
}
pub struct EmploymentAgencyIriOrLabel;
impl PartialEq<&str> for EmploymentAgencyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmploymentAgencyIri || *other == EMPLOYMENT_AGENCY_LABEL
	}
}
impl PartialEq<EmploymentAgencyIriOrLabel> for &str {
	fn eq(&self, other: &EmploymentAgencyIriOrLabel) -> bool {
		*self == EmploymentAgencyIri || *self == EMPLOYMENT_AGENCY_LABEL
	}
}
