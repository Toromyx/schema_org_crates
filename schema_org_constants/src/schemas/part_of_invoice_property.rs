/// <https://schema.org/partOfInvoice>
pub const PART_OF_INVOICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/partOfInvoice";
/// <https://schema.org/partOfInvoice>
pub const PART_OF_INVOICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partOfInvoice";
/// <https://schema.org/partOfInvoice>
pub const PART_OF_INVOICE_PROPERTY_LABEL: &str = "partOfInvoice";
pub struct PartOfInvoicePropertyIri;
impl PartialEq<&str> for PartOfInvoicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PART_OF_INVOICE_PROPERTY_IRI_HTTP || *other == PART_OF_INVOICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartOfInvoicePropertyIri> for &str {
	fn eq(&self, other: &PartOfInvoicePropertyIri) -> bool {
		*self == PART_OF_INVOICE_PROPERTY_IRI_HTTP || *self == PART_OF_INVOICE_PROPERTY_IRI_HTTPS
	}
}
pub struct PartOfInvoicePropertyIriOrLabel;
impl PartialEq<&str> for PartOfInvoicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartOfInvoicePropertyIri || *other == PART_OF_INVOICE_PROPERTY_LABEL
	}
}
impl PartialEq<PartOfInvoicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartOfInvoicePropertyIriOrLabel) -> bool {
		*self == PartOfInvoicePropertyIri || *self == PART_OF_INVOICE_PROPERTY_LABEL
	}
}
