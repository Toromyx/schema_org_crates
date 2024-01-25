/// <https://schema.org/providerMobility>
pub const PROVIDER_MOBILITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/providerMobility";
/// <https://schema.org/providerMobility>
pub const PROVIDER_MOBILITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/providerMobility";
/// <https://schema.org/providerMobility>
pub const PROVIDER_MOBILITY_PROPERTY_LABEL: &str = "providerMobility";
pub struct ProviderMobilityPropertyIri;
impl PartialEq<&str> for ProviderMobilityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROVIDER_MOBILITY_PROPERTY_IRI_HTTP
			|| *other == PROVIDER_MOBILITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProviderMobilityPropertyIri> for &str {
	fn eq(&self, other: &ProviderMobilityPropertyIri) -> bool {
		*self == PROVIDER_MOBILITY_PROPERTY_IRI_HTTP
			|| *self == PROVIDER_MOBILITY_PROPERTY_IRI_HTTPS
	}
}
pub struct ProviderMobilityPropertyIriOrLabel;
impl PartialEq<&str> for ProviderMobilityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProviderMobilityPropertyIri || *other == PROVIDER_MOBILITY_PROPERTY_LABEL
	}
}
impl PartialEq<ProviderMobilityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProviderMobilityPropertyIriOrLabel) -> bool {
		*self == ProviderMobilityPropertyIri || *self == PROVIDER_MOBILITY_PROPERTY_LABEL
	}
}
