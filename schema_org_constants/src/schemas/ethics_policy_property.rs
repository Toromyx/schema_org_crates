/// <https://schema.org/ethicsPolicy>
pub const ETHICS_POLICY_PROPERTY_IRI_HTTP: &str = "http://schema.org/ethicsPolicy";
/// <https://schema.org/ethicsPolicy>
pub const ETHICS_POLICY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/ethicsPolicy";
/// <https://schema.org/ethicsPolicy>
pub const ETHICS_POLICY_PROPERTY_LABEL: &str = "ethicsPolicy";
pub struct EthicsPolicyPropertyIri;
impl PartialEq<&str> for EthicsPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ETHICS_POLICY_PROPERTY_IRI_HTTP || *other == ETHICS_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EthicsPolicyPropertyIri> for &str {
	fn eq(&self, other: &EthicsPolicyPropertyIri) -> bool {
		*self == ETHICS_POLICY_PROPERTY_IRI_HTTP || *self == ETHICS_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct EthicsPolicyPropertyIriOrLabel;
impl PartialEq<&str> for EthicsPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EthicsPolicyPropertyIri || *other == ETHICS_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<EthicsPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EthicsPolicyPropertyIriOrLabel) -> bool {
		*self == EthicsPolicyPropertyIri || *self == ETHICS_POLICY_PROPERTY_LABEL
	}
}
