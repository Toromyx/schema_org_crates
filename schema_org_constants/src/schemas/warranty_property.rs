/// <https://schema.org/warranty>
pub const WARRANTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/warranty";
/// <https://schema.org/warranty>
pub const WARRANTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/warranty";
/// <https://schema.org/warranty>
pub const WARRANTY_PROPERTY_LABEL: &str = "warranty";
pub struct WarrantyPropertyIri;
impl PartialEq<&str> for WarrantyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WARRANTY_PROPERTY_IRI_HTTP || *other == WARRANTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<WarrantyPropertyIri> for &str {
	fn eq(&self, other: &WarrantyPropertyIri) -> bool {
		*self == WARRANTY_PROPERTY_IRI_HTTP || *self == WARRANTY_PROPERTY_IRI_HTTPS
	}
}
pub struct WarrantyPropertyIriOrLabel;
impl PartialEq<&str> for WarrantyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WarrantyPropertyIri || *other == WARRANTY_PROPERTY_LABEL
	}
}
impl PartialEq<WarrantyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &WarrantyPropertyIriOrLabel) -> bool {
		*self == WarrantyPropertyIri || *self == WARRANTY_PROPERTY_LABEL
	}
}
