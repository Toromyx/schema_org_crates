/// <https://schema.org/ListPrice>
pub const LIST_PRICE_IRI_HTTP: &str = "http://schema.org/ListPrice";
/// <https://schema.org/ListPrice>
pub const LIST_PRICE_IRI_HTTPS: &str = "https://schema.org/ListPrice";
/// <https://schema.org/ListPrice>
pub const LIST_PRICE_LABEL: &str = "ListPrice";
pub struct ListPriceIri;
impl PartialEq<&str> for ListPriceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIST_PRICE_IRI_HTTP || *other == LIST_PRICE_IRI_HTTPS
	}
}
impl PartialEq<ListPriceIri> for &str {
	fn eq(&self, other: &ListPriceIri) -> bool {
		*self == LIST_PRICE_IRI_HTTP || *self == LIST_PRICE_IRI_HTTPS
	}
}
pub struct ListPriceIriOrLabel;
impl PartialEq<&str> for ListPriceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ListPriceIri || *other == LIST_PRICE_LABEL
	}
}
impl PartialEq<ListPriceIriOrLabel> for &str {
	fn eq(&self, other: &ListPriceIriOrLabel) -> bool {
		*self == ListPriceIri || *self == LIST_PRICE_LABEL
	}
}
