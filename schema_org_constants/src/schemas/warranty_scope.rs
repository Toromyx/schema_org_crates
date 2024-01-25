/// <https://schema.org/WarrantyScope>
pub const WARRANTY_SCOPE_IRI_HTTP: &str = "http://schema.org/WarrantyScope";
/// <https://schema.org/WarrantyScope>
pub const WARRANTY_SCOPE_IRI_HTTPS: &str = "https://schema.org/WarrantyScope";
/// <https://schema.org/WarrantyScope>
pub const WARRANTY_SCOPE_LABEL: &str = "WarrantyScope";
pub struct WarrantyScopeIri;
impl PartialEq<&str> for WarrantyScopeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WARRANTY_SCOPE_IRI_HTTP || *other == WARRANTY_SCOPE_IRI_HTTPS
	}
}
impl PartialEq<WarrantyScopeIri> for &str {
	fn eq(&self, other: &WarrantyScopeIri) -> bool {
		*self == WARRANTY_SCOPE_IRI_HTTP || *self == WARRANTY_SCOPE_IRI_HTTPS
	}
}
pub struct WarrantyScopeIriOrLabel;
impl PartialEq<&str> for WarrantyScopeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WarrantyScopeIri || *other == WARRANTY_SCOPE_LABEL
	}
}
impl PartialEq<WarrantyScopeIriOrLabel> for &str {
	fn eq(&self, other: &WarrantyScopeIriOrLabel) -> bool {
		*self == WarrantyScopeIri || *self == WARRANTY_SCOPE_LABEL
	}
}
