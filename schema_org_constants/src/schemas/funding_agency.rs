/// <https://schema.org/FundingAgency>
pub const FUNDING_AGENCY_IRI_HTTP: &str = "http://schema.org/FundingAgency";
/// <https://schema.org/FundingAgency>
pub const FUNDING_AGENCY_IRI_HTTPS: &str = "https://schema.org/FundingAgency";
/// <https://schema.org/FundingAgency>
pub const FUNDING_AGENCY_LABEL: &str = "FundingAgency";
pub struct FundingAgencyIri;
impl PartialEq<&str> for FundingAgencyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUNDING_AGENCY_IRI_HTTP || *other == FUNDING_AGENCY_IRI_HTTPS
	}
}
impl PartialEq<FundingAgencyIri> for &str {
	fn eq(&self, other: &FundingAgencyIri) -> bool {
		*self == FUNDING_AGENCY_IRI_HTTP || *self == FUNDING_AGENCY_IRI_HTTPS
	}
}
pub struct FundingAgencyIriOrLabel;
impl PartialEq<&str> for FundingAgencyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FundingAgencyIri || *other == FUNDING_AGENCY_LABEL
	}
}
impl PartialEq<FundingAgencyIriOrLabel> for &str {
	fn eq(&self, other: &FundingAgencyIriOrLabel) -> bool {
		*self == FundingAgencyIri || *self == FUNDING_AGENCY_LABEL
	}
}
