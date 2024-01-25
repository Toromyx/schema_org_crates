/// <https://schema.org/sku>
pub const SKU_PROPERTY_IRI_HTTP: &str = "http://schema.org/sku";
/// <https://schema.org/sku>
pub const SKU_PROPERTY_IRI_HTTPS: &str = "https://schema.org/sku";
/// <https://schema.org/sku>
pub const SKU_PROPERTY_LABEL: &str = "sku";
pub struct SkuPropertyIri;
impl PartialEq<&str> for SkuPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SKU_PROPERTY_IRI_HTTP || *other == SKU_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SkuPropertyIri> for &str {
	fn eq(&self, other: &SkuPropertyIri) -> bool {
		*self == SKU_PROPERTY_IRI_HTTP || *self == SKU_PROPERTY_IRI_HTTPS
	}
}
pub struct SkuPropertyIriOrLabel;
impl PartialEq<&str> for SkuPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SkuPropertyIri || *other == SKU_PROPERTY_LABEL
	}
}
impl PartialEq<SkuPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SkuPropertyIriOrLabel) -> bool {
		*self == SkuPropertyIri || *self == SKU_PROPERTY_LABEL
	}
}
