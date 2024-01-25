/// <https://schema.org/FundingScheme>
pub const FUNDING_SCHEME_IRI_HTTP: &str = "http://schema.org/FundingScheme";
/// <https://schema.org/FundingScheme>
pub const FUNDING_SCHEME_IRI_HTTPS: &str = "https://schema.org/FundingScheme";
/// <https://schema.org/FundingScheme>
pub const FUNDING_SCHEME_LABEL: &str = "FundingScheme";
pub struct FundingSchemeIri;
impl PartialEq<&str> for FundingSchemeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUNDING_SCHEME_IRI_HTTP || *other == FUNDING_SCHEME_IRI_HTTPS
	}
}
impl PartialEq<FundingSchemeIri> for &str {
	fn eq(&self, other: &FundingSchemeIri) -> bool {
		*self == FUNDING_SCHEME_IRI_HTTP || *self == FUNDING_SCHEME_IRI_HTTPS
	}
}
pub struct FundingSchemeIriOrLabel;
impl PartialEq<&str> for FundingSchemeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FundingSchemeIri || *other == FUNDING_SCHEME_LABEL
	}
}
impl PartialEq<FundingSchemeIriOrLabel> for &str {
	fn eq(&self, other: &FundingSchemeIriOrLabel) -> bool {
		*self == FundingSchemeIri || *self == FUNDING_SCHEME_LABEL
	}
}
