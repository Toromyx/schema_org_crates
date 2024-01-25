/// <https://schema.org/gtin13>
pub const GTIN_13_PROPERTY_IRI_HTTP: &str = "http://schema.org/gtin13";
/// <https://schema.org/gtin13>
pub const GTIN_13_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gtin13";
/// <https://schema.org/gtin13>
pub const GTIN_13_PROPERTY_LABEL: &str = "gtin13";
pub struct Gtin13PropertyIri;
impl PartialEq<&str> for Gtin13PropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GTIN_13_PROPERTY_IRI_HTTP || *other == GTIN_13_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<Gtin13PropertyIri> for &str {
	fn eq(&self, other: &Gtin13PropertyIri) -> bool {
		*self == GTIN_13_PROPERTY_IRI_HTTP || *self == GTIN_13_PROPERTY_IRI_HTTPS
	}
}
pub struct Gtin13PropertyIriOrLabel;
impl PartialEq<&str> for Gtin13PropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Gtin13PropertyIri || *other == GTIN_13_PROPERTY_LABEL
	}
}
impl PartialEq<Gtin13PropertyIriOrLabel> for &str {
	fn eq(&self, other: &Gtin13PropertyIriOrLabel) -> bool {
		*self == Gtin13PropertyIri || *self == GTIN_13_PROPERTY_LABEL
	}
}
