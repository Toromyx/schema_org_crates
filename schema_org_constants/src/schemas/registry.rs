/// <https://schema.org/Registry>
pub const REGISTRY_IRI_HTTP: &str = "http://schema.org/Registry";
/// <https://schema.org/Registry>
pub const REGISTRY_IRI_HTTPS: &str = "https://schema.org/Registry";
/// <https://schema.org/Registry>
pub const REGISTRY_LABEL: &str = "Registry";
pub struct RegistryIri;
impl PartialEq<&str> for RegistryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REGISTRY_IRI_HTTP || *other == REGISTRY_IRI_HTTPS
	}
}
impl PartialEq<RegistryIri> for &str {
	fn eq(&self, other: &RegistryIri) -> bool {
		*self == REGISTRY_IRI_HTTP || *self == REGISTRY_IRI_HTTPS
	}
}
pub struct RegistryIriOrLabel;
impl PartialEq<&str> for RegistryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RegistryIri || *other == REGISTRY_LABEL
	}
}
impl PartialEq<RegistryIriOrLabel> for &str {
	fn eq(&self, other: &RegistryIriOrLabel) -> bool {
		*self == RegistryIri || *self == REGISTRY_LABEL
	}
}
