/// <https://schema.org/provider>
pub const PROVIDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/provider";
/// <https://schema.org/provider>
pub const PROVIDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/provider";
/// <https://schema.org/provider>
pub const PROVIDER_PROPERTY_LABEL: &str = "provider";
pub struct ProviderPropertyIri;
impl PartialEq<&str> for ProviderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROVIDER_PROPERTY_IRI_HTTP || *other == PROVIDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProviderPropertyIri> for &str {
	fn eq(&self, other: &ProviderPropertyIri) -> bool {
		*self == PROVIDER_PROPERTY_IRI_HTTP || *self == PROVIDER_PROPERTY_IRI_HTTPS
	}
}
pub struct ProviderPropertyIriOrLabel;
impl PartialEq<&str> for ProviderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProviderPropertyIri || *other == PROVIDER_PROPERTY_LABEL
	}
}
impl PartialEq<ProviderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProviderPropertyIriOrLabel) -> bool {
		*self == ProviderPropertyIri || *self == PROVIDER_PROPERTY_LABEL
	}
}
