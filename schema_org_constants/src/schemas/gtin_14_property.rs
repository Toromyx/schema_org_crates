/// <https://schema.org/gtin14>
pub const GTIN_14_PROPERTY_IRI_HTTP: &str = "http://schema.org/gtin14";
/// <https://schema.org/gtin14>
pub const GTIN_14_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gtin14";
/// <https://schema.org/gtin14>
pub const GTIN_14_PROPERTY_LABEL: &str = "gtin14";
pub struct Gtin14PropertyIri;
impl PartialEq<&str> for Gtin14PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GTIN_14_PROPERTY_IRI_HTTP || *other == GTIN_14_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Gtin14PropertyIri> for &str {
	fn eq(&self, other: &Gtin14PropertyIri) -> bool {
		*self == GTIN_14_PROPERTY_IRI_HTTP || *self == GTIN_14_PROPERTY_IRI_HTTPS
	}
}
pub struct Gtin14PropertyIriOrLabel;
impl PartialEq<&str> for Gtin14PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Gtin14PropertyIri || *other == GTIN_14_PROPERTY_LABEL
	}
}
impl PartialEq<Gtin14PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Gtin14PropertyIriOrLabel) -> bool {
		*self == Gtin14PropertyIri || *self == GTIN_14_PROPERTY_LABEL
	}
}
