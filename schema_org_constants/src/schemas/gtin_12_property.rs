/// <https://schema.org/gtin12>
pub const GTIN_12_PROPERTY_IRI_HTTP: &str = "http://schema.org/gtin12";
/// <https://schema.org/gtin12>
pub const GTIN_12_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gtin12";
/// <https://schema.org/gtin12>
pub const GTIN_12_PROPERTY_LABEL: &str = "gtin12";
pub struct Gtin12PropertyIri;
impl PartialEq<&str> for Gtin12PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GTIN_12_PROPERTY_IRI_HTTP || *other == GTIN_12_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Gtin12PropertyIri> for &str {
	fn eq(&self, other: &Gtin12PropertyIri) -> bool {
		*self == GTIN_12_PROPERTY_IRI_HTTP || *self == GTIN_12_PROPERTY_IRI_HTTPS
	}
}
pub struct Gtin12PropertyIriOrLabel;
impl PartialEq<&str> for Gtin12PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Gtin12PropertyIri || *other == GTIN_12_PROPERTY_LABEL
	}
}
impl PartialEq<Gtin12PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Gtin12PropertyIriOrLabel) -> bool {
		*self == Gtin12PropertyIri || *self == GTIN_12_PROPERTY_LABEL
	}
}
