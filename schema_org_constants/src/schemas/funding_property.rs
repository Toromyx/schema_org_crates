/// <https://schema.org/funding>
pub const FUNDING_PROPERTY_IRI_HTTP: &str = "http://schema.org/funding";
/// <https://schema.org/funding>
pub const FUNDING_PROPERTY_IRI_HTTPS: &str = "https://schema.org/funding";
/// <https://schema.org/funding>
pub const FUNDING_PROPERTY_LABEL: &str = "funding";
pub struct FundingPropertyIri;
impl PartialEq<&str> for FundingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUNDING_PROPERTY_IRI_HTTP || *other == FUNDING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FundingPropertyIri> for &str {
	fn eq(&self, other: &FundingPropertyIri) -> bool {
		*self == FUNDING_PROPERTY_IRI_HTTP || *self == FUNDING_PROPERTY_IRI_HTTPS
	}
}
pub struct FundingPropertyIriOrLabel;
impl PartialEq<&str> for FundingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FundingPropertyIri || *other == FUNDING_PROPERTY_LABEL
	}
}
impl PartialEq<FundingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FundingPropertyIriOrLabel) -> bool {
		*self == FundingPropertyIri || *self == FUNDING_PROPERTY_LABEL
	}
}
