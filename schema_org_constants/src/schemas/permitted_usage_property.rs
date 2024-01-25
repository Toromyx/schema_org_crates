/// <https://schema.org/permittedUsage>
pub const PERMITTED_USAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/permittedUsage";
/// <https://schema.org/permittedUsage>
pub const PERMITTED_USAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/permittedUsage";
/// <https://schema.org/permittedUsage>
pub const PERMITTED_USAGE_PROPERTY_LABEL: &str = "permittedUsage";
pub struct PermittedUsagePropertyIri;
impl PartialEq<&str> for PermittedUsagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERMITTED_USAGE_PROPERTY_IRI_HTTP || *other == PERMITTED_USAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PermittedUsagePropertyIri> for &str {
	fn eq(&self, other: &PermittedUsagePropertyIri) -> bool {
		*self == PERMITTED_USAGE_PROPERTY_IRI_HTTP || *self == PERMITTED_USAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct PermittedUsagePropertyIriOrLabel;
impl PartialEq<&str> for PermittedUsagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PermittedUsagePropertyIri || *other == PERMITTED_USAGE_PROPERTY_LABEL
	}
}
impl PartialEq<PermittedUsagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PermittedUsagePropertyIriOrLabel) -> bool {
		*self == PermittedUsagePropertyIri || *self == PERMITTED_USAGE_PROPERTY_LABEL
	}
}
