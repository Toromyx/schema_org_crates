/// <https://schema.org/gtin>
pub const GTIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/gtin";
/// <https://schema.org/gtin>
pub const GTIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gtin";
/// <https://schema.org/gtin>
pub const GTIN_PROPERTY_LABEL: &str = "gtin";
pub struct GtinPropertyIri;
impl PartialEq<&str> for GtinPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GTIN_PROPERTY_IRI_HTTP || *other == GTIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GtinPropertyIri> for &str {
	fn eq(&self, other: &GtinPropertyIri) -> bool {
		*self == GTIN_PROPERTY_IRI_HTTP || *self == GTIN_PROPERTY_IRI_HTTPS
	}
}
pub struct GtinPropertyIriOrLabel;
impl PartialEq<&str> for GtinPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GtinPropertyIri || *other == GTIN_PROPERTY_LABEL
	}
}
impl PartialEq<GtinPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GtinPropertyIriOrLabel) -> bool {
		*self == GtinPropertyIri || *self == GTIN_PROPERTY_LABEL
	}
}
