/// <https://schema.org/gtin8>
pub const GTIN_8_PROPERTY_IRI_HTTP: &str = "http://schema.org/gtin8";
/// <https://schema.org/gtin8>
pub const GTIN_8_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gtin8";
/// <https://schema.org/gtin8>
pub const GTIN_8_PROPERTY_LABEL: &str = "gtin8";
pub struct Gtin8PropertyIri;
impl PartialEq<&str> for Gtin8PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GTIN_8_PROPERTY_IRI_HTTP || *other == GTIN_8_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Gtin8PropertyIri> for &str {
	fn eq(&self, other: &Gtin8PropertyIri) -> bool {
		*self == GTIN_8_PROPERTY_IRI_HTTP || *self == GTIN_8_PROPERTY_IRI_HTTPS
	}
}
pub struct Gtin8PropertyIriOrLabel;
impl PartialEq<&str> for Gtin8PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Gtin8PropertyIri || *other == GTIN_8_PROPERTY_LABEL
	}
}
impl PartialEq<Gtin8PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Gtin8PropertyIriOrLabel) -> bool {
		*self == Gtin8PropertyIri || *self == GTIN_8_PROPERTY_LABEL
	}
}
