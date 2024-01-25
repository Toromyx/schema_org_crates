/// <https://schema.org/annualPercentageRate>
pub const ANNUAL_PERCENTAGE_RATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/annualPercentageRate";
/// <https://schema.org/annualPercentageRate>
pub const ANNUAL_PERCENTAGE_RATE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/annualPercentageRate";
/// <https://schema.org/annualPercentageRate>
pub const ANNUAL_PERCENTAGE_RATE_PROPERTY_LABEL: &str = "annualPercentageRate";
pub struct AnnualPercentageRatePropertyIri;
impl PartialEq<&str> for AnnualPercentageRatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANNUAL_PERCENTAGE_RATE_PROPERTY_IRI_HTTP
			|| *other == ANNUAL_PERCENTAGE_RATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AnnualPercentageRatePropertyIri> for &str {
	fn eq(&self, other: &AnnualPercentageRatePropertyIri) -> bool {
		*self == ANNUAL_PERCENTAGE_RATE_PROPERTY_IRI_HTTP
			|| *self == ANNUAL_PERCENTAGE_RATE_PROPERTY_IRI_HTTPS
	}
}
pub struct AnnualPercentageRatePropertyIriOrLabel;
impl PartialEq<&str> for AnnualPercentageRatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnnualPercentageRatePropertyIri || *other == ANNUAL_PERCENTAGE_RATE_PROPERTY_LABEL
	}
}
impl PartialEq<AnnualPercentageRatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AnnualPercentageRatePropertyIriOrLabel) -> bool {
		*self == AnnualPercentageRatePropertyIri || *self == ANNUAL_PERCENTAGE_RATE_PROPERTY_LABEL
	}
}
