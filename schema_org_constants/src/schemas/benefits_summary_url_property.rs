/// <https://schema.org/benefitsSummaryUrl>
pub const BENEFITS_SUMMARY_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/benefitsSummaryUrl";
/// <https://schema.org/benefitsSummaryUrl>
pub const BENEFITS_SUMMARY_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/benefitsSummaryUrl";
/// <https://schema.org/benefitsSummaryUrl>
pub const BENEFITS_SUMMARY_URL_PROPERTY_LABEL: &str = "benefitsSummaryUrl";
pub struct BenefitsSummaryUrlPropertyIri;
impl PartialEq<&str> for BenefitsSummaryUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BENEFITS_SUMMARY_URL_PROPERTY_IRI_HTTP
			|| *other == BENEFITS_SUMMARY_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BenefitsSummaryUrlPropertyIri> for &str {
	fn eq(&self, other: &BenefitsSummaryUrlPropertyIri) -> bool {
		*self == BENEFITS_SUMMARY_URL_PROPERTY_IRI_HTTP
			|| *self == BENEFITS_SUMMARY_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct BenefitsSummaryUrlPropertyIriOrLabel;
impl PartialEq<&str> for BenefitsSummaryUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BenefitsSummaryUrlPropertyIri || *other == BENEFITS_SUMMARY_URL_PROPERTY_LABEL
	}
}
impl PartialEq<BenefitsSummaryUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BenefitsSummaryUrlPropertyIriOrLabel) -> bool {
		*self == BenefitsSummaryUrlPropertyIri || *self == BENEFITS_SUMMARY_URL_PROPERTY_LABEL
	}
}
