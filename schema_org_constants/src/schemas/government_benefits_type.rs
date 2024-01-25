/// <https://schema.org/GovernmentBenefitsType>
pub const GOVERNMENT_BENEFITS_TYPE_IRI_HTTP: &str = "http://schema.org/GovernmentBenefitsType";
/// <https://schema.org/GovernmentBenefitsType>
pub const GOVERNMENT_BENEFITS_TYPE_IRI_HTTPS: &str = "https://schema.org/GovernmentBenefitsType";
/// <https://schema.org/GovernmentBenefitsType>
pub const GOVERNMENT_BENEFITS_TYPE_LABEL: &str = "GovernmentBenefitsType";
pub struct GovernmentBenefitsTypeIri;
impl PartialEq<&str> for GovernmentBenefitsTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GOVERNMENT_BENEFITS_TYPE_IRI_HTTP || *other == GOVERNMENT_BENEFITS_TYPE_IRI_HTTPS
	}
}
impl PartialEq<GovernmentBenefitsTypeIri> for &str {
	fn eq(&self, other: &GovernmentBenefitsTypeIri) -> bool {
		*self == GOVERNMENT_BENEFITS_TYPE_IRI_HTTP || *self == GOVERNMENT_BENEFITS_TYPE_IRI_HTTPS
	}
}
pub struct GovernmentBenefitsTypeIriOrLabel;
impl PartialEq<&str> for GovernmentBenefitsTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GovernmentBenefitsTypeIri || *other == GOVERNMENT_BENEFITS_TYPE_LABEL
	}
}
impl PartialEq<GovernmentBenefitsTypeIriOrLabel> for &str {
	fn eq(&self, other: &GovernmentBenefitsTypeIriOrLabel) -> bool {
		*self == GovernmentBenefitsTypeIri || *self == GOVERNMENT_BENEFITS_TYPE_LABEL
	}
}
