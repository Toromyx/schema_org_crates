/// <https://schema.org/Wholesale>
pub const WHOLESALE_IRI_HTTP: &str = "http://schema.org/Wholesale";
/// <https://schema.org/Wholesale>
pub const WHOLESALE_IRI_HTTPS: &str = "https://schema.org/Wholesale";
/// <https://schema.org/Wholesale>
pub const WHOLESALE_LABEL: &str = "Wholesale";
pub struct WholesaleIri;
impl PartialEq<&str> for WholesaleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WHOLESALE_IRI_HTTP || *other == WHOLESALE_IRI_HTTPS
	}
}
impl PartialEq<WholesaleIri> for &str {
	fn eq(&self, other: &WholesaleIri) -> bool {
		*self == WHOLESALE_IRI_HTTP || *self == WHOLESALE_IRI_HTTPS
	}
}
pub struct WholesaleIriOrLabel;
impl PartialEq<&str> for WholesaleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WholesaleIri || *other == WHOLESALE_LABEL
	}
}
impl PartialEq<WholesaleIriOrLabel> for &str {
	fn eq(&self, other: &WholesaleIriOrLabel) -> bool {
		*self == WholesaleIri || *self == WHOLESALE_LABEL
	}
}
