/// <https://schema.org/SaleEvent>
pub const SALE_EVENT_IRI_HTTP: &str = "http://schema.org/SaleEvent";
/// <https://schema.org/SaleEvent>
pub const SALE_EVENT_IRI_HTTPS: &str = "https://schema.org/SaleEvent";
/// <https://schema.org/SaleEvent>
pub const SALE_EVENT_LABEL: &str = "SaleEvent";
pub struct SaleEventIri;
impl PartialEq<&str> for SaleEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SALE_EVENT_IRI_HTTP || *other == SALE_EVENT_IRI_HTTPS
	}
}
impl PartialEq<SaleEventIri> for &str {
	fn eq(&self, other: &SaleEventIri) -> bool {
		*self == SALE_EVENT_IRI_HTTP || *self == SALE_EVENT_IRI_HTTPS
	}
}
pub struct SaleEventIriOrLabel;
impl PartialEq<&str> for SaleEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SaleEventIri || *other == SALE_EVENT_LABEL
	}
}
impl PartialEq<SaleEventIriOrLabel> for &str {
	fn eq(&self, other: &SaleEventIriOrLabel) -> bool {
		*self == SaleEventIri || *self == SALE_EVENT_LABEL
	}
}
