/// <https://schema.org/InvoicePrice>
pub const INVOICE_PRICE_IRI_HTTP: &str = "http://schema.org/InvoicePrice";
/// <https://schema.org/InvoicePrice>
pub const INVOICE_PRICE_IRI_HTTPS: &str = "https://schema.org/InvoicePrice";
/// <https://schema.org/InvoicePrice>
pub const INVOICE_PRICE_LABEL: &str = "InvoicePrice";
pub struct InvoicePriceIri;
impl PartialEq<&str> for InvoicePriceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INVOICE_PRICE_IRI_HTTP || *other == INVOICE_PRICE_IRI_HTTPS
	}
}
impl PartialEq<InvoicePriceIri> for &str {
	fn eq(&self, other: &InvoicePriceIri) -> bool {
		*self == INVOICE_PRICE_IRI_HTTP || *self == INVOICE_PRICE_IRI_HTTPS
	}
}
pub struct InvoicePriceIriOrLabel;
impl PartialEq<&str> for InvoicePriceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InvoicePriceIri || *other == INVOICE_PRICE_LABEL
	}
}
impl PartialEq<InvoicePriceIriOrLabel> for &str {
	fn eq(&self, other: &InvoicePriceIriOrLabel) -> bool {
		*self == InvoicePriceIri || *self == INVOICE_PRICE_LABEL
	}
}
