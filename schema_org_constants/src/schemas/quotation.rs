/// <https://schema.org/Quotation>
pub const QUOTATION_IRI_HTTP: &str = "http://schema.org/Quotation";
/// <https://schema.org/Quotation>
pub const QUOTATION_IRI_HTTPS: &str = "https://schema.org/Quotation";
/// <https://schema.org/Quotation>
pub const QUOTATION_LABEL: &str = "Quotation";
pub struct QuotationIri;
impl PartialEq<&str> for QuotationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUOTATION_IRI_HTTP || *other == QUOTATION_IRI_HTTPS
	}
}
impl PartialEq<QuotationIri> for &str {
	fn eq(&self, other: &QuotationIri) -> bool {
		*self == QUOTATION_IRI_HTTP || *self == QUOTATION_IRI_HTTPS
	}
}
pub struct QuotationIriOrLabel;
impl PartialEq<&str> for QuotationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuotationIri || *other == QUOTATION_LABEL
	}
}
impl PartialEq<QuotationIriOrLabel> for &str {
	fn eq(&self, other: &QuotationIriOrLabel) -> bool {
		*self == QuotationIri || *self == QUOTATION_LABEL
	}
}
