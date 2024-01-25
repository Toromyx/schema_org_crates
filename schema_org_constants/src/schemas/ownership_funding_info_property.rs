/// <https://schema.org/ownershipFundingInfo>
pub const OWNERSHIP_FUNDING_INFO_PROPERTY_IRI_HTTP: &str = "http://schema.org/ownershipFundingInfo";
/// <https://schema.org/ownershipFundingInfo>
pub const OWNERSHIP_FUNDING_INFO_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/ownershipFundingInfo";
/// <https://schema.org/ownershipFundingInfo>
pub const OWNERSHIP_FUNDING_INFO_PROPERTY_LABEL: &str = "ownershipFundingInfo";
pub struct OwnershipFundingInfoPropertyIri;
impl PartialEq<&str> for OwnershipFundingInfoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OWNERSHIP_FUNDING_INFO_PROPERTY_IRI_HTTP
			|| *other == OWNERSHIP_FUNDING_INFO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OwnershipFundingInfoPropertyIri> for &str {
	fn eq(&self, other: &OwnershipFundingInfoPropertyIri) -> bool {
		*self == OWNERSHIP_FUNDING_INFO_PROPERTY_IRI_HTTP
			|| *self == OWNERSHIP_FUNDING_INFO_PROPERTY_IRI_HTTPS
	}
}
pub struct OwnershipFundingInfoPropertyIriOrLabel;
impl PartialEq<&str> for OwnershipFundingInfoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OwnershipFundingInfoPropertyIri || *other == OWNERSHIP_FUNDING_INFO_PROPERTY_LABEL
	}
}
impl PartialEq<OwnershipFundingInfoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OwnershipFundingInfoPropertyIriOrLabel) -> bool {
		*self == OwnershipFundingInfoPropertyIri || *self == OWNERSHIP_FUNDING_INFO_PROPERTY_LABEL
	}
}
