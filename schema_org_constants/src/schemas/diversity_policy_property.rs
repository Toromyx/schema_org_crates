/// <https://schema.org/diversityPolicy>
pub const DIVERSITY_POLICY_PROPERTY_IRI_HTTP: &str = "http://schema.org/diversityPolicy";
/// <https://schema.org/diversityPolicy>
pub const DIVERSITY_POLICY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/diversityPolicy";
/// <https://schema.org/diversityPolicy>
pub const DIVERSITY_POLICY_PROPERTY_LABEL: &str = "diversityPolicy";
pub struct DiversityPolicyPropertyIri;
impl PartialEq<&str> for DiversityPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIVERSITY_POLICY_PROPERTY_IRI_HTTP
			|| *other == DIVERSITY_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiversityPolicyPropertyIri> for &str {
	fn eq(&self, other: &DiversityPolicyPropertyIri) -> bool {
		*self == DIVERSITY_POLICY_PROPERTY_IRI_HTTP || *self == DIVERSITY_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct DiversityPolicyPropertyIriOrLabel;
impl PartialEq<&str> for DiversityPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiversityPolicyPropertyIri || *other == DIVERSITY_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<DiversityPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiversityPolicyPropertyIriOrLabel) -> bool {
		*self == DiversityPolicyPropertyIri || *self == DIVERSITY_POLICY_PROPERTY_LABEL
	}
}
