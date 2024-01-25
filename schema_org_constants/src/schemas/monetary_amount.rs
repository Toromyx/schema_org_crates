/// <https://schema.org/MonetaryAmount>
pub const MONETARY_AMOUNT_IRI_HTTP: &str = "http://schema.org/MonetaryAmount";
/// <https://schema.org/MonetaryAmount>
pub const MONETARY_AMOUNT_IRI_HTTPS: &str = "https://schema.org/MonetaryAmount";
/// <https://schema.org/MonetaryAmount>
pub const MONETARY_AMOUNT_LABEL: &str = "MonetaryAmount";
pub struct MonetaryAmountIri;
impl PartialEq<&str> for MonetaryAmountIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MONETARY_AMOUNT_IRI_HTTP || *other == MONETARY_AMOUNT_IRI_HTTPS
	}
}
impl PartialEq<MonetaryAmountIri> for &str {
	fn eq(&self, other: &MonetaryAmountIri) -> bool {
		*self == MONETARY_AMOUNT_IRI_HTTP || *self == MONETARY_AMOUNT_IRI_HTTPS
	}
}
pub struct MonetaryAmountIriOrLabel;
impl PartialEq<&str> for MonetaryAmountIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MonetaryAmountIri || *other == MONETARY_AMOUNT_LABEL
	}
}
impl PartialEq<MonetaryAmountIriOrLabel> for &str {
	fn eq(&self, other: &MonetaryAmountIriOrLabel) -> bool {
		*self == MonetaryAmountIri || *self == MONETARY_AMOUNT_LABEL
	}
}
