/// <https://schema.org/Brand>
pub const BRAND_IRI_HTTP: &str = "http://schema.org/Brand";
/// <https://schema.org/Brand>
pub const BRAND_IRI_HTTPS: &str = "https://schema.org/Brand";
/// <https://schema.org/Brand>
pub const BRAND_LABEL: &str = "Brand";
pub struct BrandIri;
impl PartialEq<&str> for BrandIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BRAND_IRI_HTTP || *other == BRAND_IRI_HTTPS
	}
}
impl PartialEq<BrandIri> for &str {
	fn eq(&self, other: &BrandIri) -> bool {
		*self == BRAND_IRI_HTTP || *self == BRAND_IRI_HTTPS
	}
}
pub struct BrandIriOrLabel;
impl PartialEq<&str> for BrandIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BrandIri || *other == BRAND_LABEL
	}
}
impl PartialEq<BrandIriOrLabel> for &str {
	fn eq(&self, other: &BrandIriOrLabel) -> bool {
		*self == BrandIri || *self == BRAND_LABEL
	}
}
