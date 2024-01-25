/// <https://schema.org/warrantyScope>
pub const WARRANTY_SCOPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/warrantyScope";
/// <https://schema.org/warrantyScope>
pub const WARRANTY_SCOPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/warrantyScope";
/// <https://schema.org/warrantyScope>
pub const WARRANTY_SCOPE_PROPERTY_LABEL: &str = "warrantyScope";
pub struct WarrantyScopePropertyIri;
impl PartialEq<&str> for WarrantyScopePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WARRANTY_SCOPE_PROPERTY_IRI_HTTP || *other == WARRANTY_SCOPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WarrantyScopePropertyIri> for &str {
	fn eq(&self, other: &WarrantyScopePropertyIri) -> bool {
		*self == WARRANTY_SCOPE_PROPERTY_IRI_HTTP || *self == WARRANTY_SCOPE_PROPERTY_IRI_HTTPS
	}
}
pub struct WarrantyScopePropertyIriOrLabel;
impl PartialEq<&str> for WarrantyScopePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WarrantyScopePropertyIri || *other == WARRANTY_SCOPE_PROPERTY_LABEL
	}
}
impl PartialEq<WarrantyScopePropertyIriOrLabel> for &str {
	fn eq(&self, other: &WarrantyScopePropertyIriOrLabel) -> bool {
		*self == WarrantyScopePropertyIri || *self == WARRANTY_SCOPE_PROPERTY_LABEL
	}
}
