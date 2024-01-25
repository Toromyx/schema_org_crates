/// <https://schema.org/Invoice>
pub const INVOICE_IRI_HTTP: &str = "http://schema.org/Invoice";
/// <https://schema.org/Invoice>
pub const INVOICE_IRI_HTTPS: &str = "https://schema.org/Invoice";
/// <https://schema.org/Invoice>
pub const INVOICE_LABEL: &str = "Invoice";
pub struct InvoiceIri;
impl PartialEq<&str> for InvoiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INVOICE_IRI_HTTP || *other == INVOICE_IRI_HTTPS
	}
}
impl PartialEq<InvoiceIri> for &str {
	fn eq(&self, other: &InvoiceIri) -> bool {
		*self == INVOICE_IRI_HTTP || *self == INVOICE_IRI_HTTPS
	}
}
pub struct InvoiceIriOrLabel;
impl PartialEq<&str> for InvoiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InvoiceIri || *other == INVOICE_LABEL
	}
}
impl PartialEq<InvoiceIriOrLabel> for &str {
	fn eq(&self, other: &InvoiceIriOrLabel) -> bool {
		*self == InvoiceIri || *self == INVOICE_LABEL
	}
}
