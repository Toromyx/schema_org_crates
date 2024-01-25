/// <https://schema.org/benefits>
#[deprecated = "This schema is superseded by <https://schema.org/jobBenefits>."]
pub const BENEFITS_PROPERTY_IRI_HTTP: &str = "http://schema.org/benefits";
/// <https://schema.org/benefits>
#[deprecated = "This schema is superseded by <https://schema.org/jobBenefits>."]
pub const BENEFITS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/benefits";
/// <https://schema.org/benefits>
#[deprecated = "This schema is superseded by <https://schema.org/jobBenefits>."]
pub const BENEFITS_PROPERTY_LABEL: &str = "benefits";
pub struct BenefitsPropertyIri;
impl PartialEq<&str> for BenefitsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BENEFITS_PROPERTY_IRI_HTTP || *other == BENEFITS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BenefitsPropertyIri> for &str {
	fn eq(&self, other: &BenefitsPropertyIri) -> bool {
		*self == BENEFITS_PROPERTY_IRI_HTTP || *self == BENEFITS_PROPERTY_IRI_HTTPS
	}
}
pub struct BenefitsPropertyIriOrLabel;
impl PartialEq<&str> for BenefitsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BenefitsPropertyIri || *other == BENEFITS_PROPERTY_LABEL
	}
}
impl PartialEq<BenefitsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BenefitsPropertyIriOrLabel) -> bool {
		*self == BenefitsPropertyIri || *self == BENEFITS_PROPERTY_LABEL
	}
}
