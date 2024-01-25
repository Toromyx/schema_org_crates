/// <https://schema.org/governmentBenefitsInfo>
pub const GOVERNMENT_BENEFITS_INFO_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/governmentBenefitsInfo";
/// <https://schema.org/governmentBenefitsInfo>
pub const GOVERNMENT_BENEFITS_INFO_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/governmentBenefitsInfo";
/// <https://schema.org/governmentBenefitsInfo>
pub const GOVERNMENT_BENEFITS_INFO_PROPERTY_LABEL: &str = "governmentBenefitsInfo";
pub struct GovernmentBenefitsInfoPropertyIri;
impl PartialEq<&str> for GovernmentBenefitsInfoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GOVERNMENT_BENEFITS_INFO_PROPERTY_IRI_HTTP
			|| *other == GOVERNMENT_BENEFITS_INFO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GovernmentBenefitsInfoPropertyIri> for &str {
	fn eq(&self, other: &GovernmentBenefitsInfoPropertyIri) -> bool {
		*self == GOVERNMENT_BENEFITS_INFO_PROPERTY_IRI_HTTP
			|| *self == GOVERNMENT_BENEFITS_INFO_PROPERTY_IRI_HTTPS
	}
}
pub struct GovernmentBenefitsInfoPropertyIriOrLabel;
impl PartialEq<&str> for GovernmentBenefitsInfoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GovernmentBenefitsInfoPropertyIri
			|| *other == GOVERNMENT_BENEFITS_INFO_PROPERTY_LABEL
	}
}
impl PartialEq<GovernmentBenefitsInfoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GovernmentBenefitsInfoPropertyIriOrLabel) -> bool {
		*self == GovernmentBenefitsInfoPropertyIri
			|| *self == GOVERNMENT_BENEFITS_INFO_PROPERTY_LABEL
	}
}
