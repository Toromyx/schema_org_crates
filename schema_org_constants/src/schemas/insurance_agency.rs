/// <https://schema.org/InsuranceAgency>
pub const INSURANCE_AGENCY_IRI_HTTP: &str = "http://schema.org/InsuranceAgency";
/// <https://schema.org/InsuranceAgency>
pub const INSURANCE_AGENCY_IRI_HTTPS: &str = "https://schema.org/InsuranceAgency";
/// <https://schema.org/InsuranceAgency>
pub const INSURANCE_AGENCY_LABEL: &str = "InsuranceAgency";
pub struct InsuranceAgencyIri;
impl PartialEq<&str> for InsuranceAgencyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INSURANCE_AGENCY_IRI_HTTP || *other == INSURANCE_AGENCY_IRI_HTTPS
	}
}
impl PartialEq<InsuranceAgencyIri> for &str {
	fn eq(&self, other: &InsuranceAgencyIri) -> bool {
		*self == INSURANCE_AGENCY_IRI_HTTP || *self == INSURANCE_AGENCY_IRI_HTTPS
	}
}
pub struct InsuranceAgencyIriOrLabel;
impl PartialEq<&str> for InsuranceAgencyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InsuranceAgencyIri || *other == INSURANCE_AGENCY_LABEL
	}
}
impl PartialEq<InsuranceAgencyIriOrLabel> for &str {
	fn eq(&self, other: &InsuranceAgencyIriOrLabel) -> bool {
		*self == InsuranceAgencyIri || *self == INSURANCE_AGENCY_LABEL
	}
}
