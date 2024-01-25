/// <https://schema.org/amount>
pub const AMOUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/amount";
/// <https://schema.org/amount>
pub const AMOUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/amount";
/// <https://schema.org/amount>
pub const AMOUNT_PROPERTY_LABEL: &str = "amount";
pub struct AmountPropertyIri;
impl PartialEq<&str> for AmountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AMOUNT_PROPERTY_IRI_HTTP || *other == AMOUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AmountPropertyIri> for &str {
	fn eq(&self, other: &AmountPropertyIri) -> bool {
		*self == AMOUNT_PROPERTY_IRI_HTTP || *self == AMOUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct AmountPropertyIriOrLabel;
impl PartialEq<&str> for AmountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AmountPropertyIri || *other == AMOUNT_PROPERTY_LABEL
	}
}
impl PartialEq<AmountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AmountPropertyIriOrLabel) -> bool {
		*self == AmountPropertyIri || *self == AMOUNT_PROPERTY_LABEL
	}
}
