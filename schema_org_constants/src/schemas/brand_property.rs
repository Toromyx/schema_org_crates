/// <https://schema.org/brand>
pub const BRAND_PROPERTY_IRI_HTTP: &str = "http://schema.org/brand";
/// <https://schema.org/brand>
pub const BRAND_PROPERTY_IRI_HTTPS: &str = "https://schema.org/brand";
/// <https://schema.org/brand>
pub const BRAND_PROPERTY_LABEL: &str = "brand";
pub struct BrandPropertyIri;
impl PartialEq<&str> for BrandPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BRAND_PROPERTY_IRI_HTTP || *other == BRAND_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BrandPropertyIri> for &str {
	fn eq(&self, other: &BrandPropertyIri) -> bool {
		*self == BRAND_PROPERTY_IRI_HTTP || *self == BRAND_PROPERTY_IRI_HTTPS
	}
}
pub struct BrandPropertyIriOrLabel;
impl PartialEq<&str> for BrandPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BrandPropertyIri || *other == BRAND_PROPERTY_LABEL
	}
}
impl PartialEq<BrandPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BrandPropertyIriOrLabel) -> bool {
		*self == BrandPropertyIri || *self == BRAND_PROPERTY_LABEL
	}
}
