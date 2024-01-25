/// <https://schema.org/incentives>
#[deprecated = "This schema is superseded by <https://schema.org/incentiveCompensation>."]
pub const INCENTIVES_PROPERTY_IRI_HTTP: &str = "http://schema.org/incentives";
/// <https://schema.org/incentives>
#[deprecated = "This schema is superseded by <https://schema.org/incentiveCompensation>."]
pub const INCENTIVES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/incentives";
/// <https://schema.org/incentives>
#[deprecated = "This schema is superseded by <https://schema.org/incentiveCompensation>."]
pub const INCENTIVES_PROPERTY_LABEL: &str = "incentives";
pub struct IncentivesPropertyIri;
impl PartialEq<&str> for IncentivesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCENTIVES_PROPERTY_IRI_HTTP || *other == INCENTIVES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncentivesPropertyIri> for &str {
	fn eq(&self, other: &IncentivesPropertyIri) -> bool {
		*self == INCENTIVES_PROPERTY_IRI_HTTP || *self == INCENTIVES_PROPERTY_IRI_HTTPS
	}
}
pub struct IncentivesPropertyIriOrLabel;
impl PartialEq<&str> for IncentivesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncentivesPropertyIri || *other == INCENTIVES_PROPERTY_LABEL
	}
}
impl PartialEq<IncentivesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncentivesPropertyIriOrLabel) -> bool {
		*self == IncentivesPropertyIri || *self == INCENTIVES_PROPERTY_LABEL
	}
}
