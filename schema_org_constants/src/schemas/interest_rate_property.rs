/// <https://schema.org/interestRate>
pub const INTEREST_RATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/interestRate";
/// <https://schema.org/interestRate>
pub const INTEREST_RATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/interestRate";
/// <https://schema.org/interestRate>
pub const INTEREST_RATE_PROPERTY_LABEL: &str = "interestRate";
pub struct InterestRatePropertyIri;
impl PartialEq<&str> for InterestRatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTEREST_RATE_PROPERTY_IRI_HTTP || *other == INTEREST_RATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InterestRatePropertyIri> for &str {
	fn eq(&self, other: &InterestRatePropertyIri) -> bool {
		*self == INTEREST_RATE_PROPERTY_IRI_HTTP || *self == INTEREST_RATE_PROPERTY_IRI_HTTPS
	}
}
pub struct InterestRatePropertyIriOrLabel;
impl PartialEq<&str> for InterestRatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InterestRatePropertyIri || *other == INTEREST_RATE_PROPERTY_LABEL
	}
}
impl PartialEq<InterestRatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &InterestRatePropertyIriOrLabel) -> bool {
		*self == InterestRatePropertyIri || *self == INTEREST_RATE_PROPERTY_LABEL
	}
}
