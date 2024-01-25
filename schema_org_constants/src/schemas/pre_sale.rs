/// <https://schema.org/PreSale>
pub const PRE_SALE_IRI_HTTP: &str = "http://schema.org/PreSale";
/// <https://schema.org/PreSale>
pub const PRE_SALE_IRI_HTTPS: &str = "https://schema.org/PreSale";
/// <https://schema.org/PreSale>
pub const PRE_SALE_LABEL: &str = "PreSale";
pub struct PreSaleIri;
impl PartialEq<&str> for PreSaleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRE_SALE_IRI_HTTP || *other == PRE_SALE_IRI_HTTPS
	}
}
impl PartialEq<PreSaleIri> for &str {
	fn eq(&self, other: &PreSaleIri) -> bool {
		*self == PRE_SALE_IRI_HTTP || *self == PRE_SALE_IRI_HTTPS
	}
}
pub struct PreSaleIriOrLabel;
impl PartialEq<&str> for PreSaleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreSaleIri || *other == PRE_SALE_LABEL
	}
}
impl PartialEq<PreSaleIriOrLabel> for &str {
	fn eq(&self, other: &PreSaleIriOrLabel) -> bool {
		*self == PreSaleIri || *self == PRE_SALE_LABEL
	}
}
