/// <https://schema.org/Quantity>
pub const QUANTITY_IRI_HTTP: &str = "http://schema.org/Quantity";
/// <https://schema.org/Quantity>
pub const QUANTITY_IRI_HTTPS: &str = "https://schema.org/Quantity";
/// <https://schema.org/Quantity>
pub const QUANTITY_LABEL: &str = "Quantity";
pub struct QuantityIri;
impl PartialEq<&str> for QuantityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUANTITY_IRI_HTTP || *other == QUANTITY_IRI_HTTPS
	}
}
impl PartialEq<QuantityIri> for &str {
	fn eq(&self, other: &QuantityIri) -> bool {
		*self == QUANTITY_IRI_HTTP || *self == QUANTITY_IRI_HTTPS
	}
}
pub struct QuantityIriOrLabel;
impl PartialEq<&str> for QuantityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuantityIri || *other == QUANTITY_LABEL
	}
}
impl PartialEq<QuantityIriOrLabel> for &str {
	fn eq(&self, other: &QuantityIriOrLabel) -> bool {
		*self == QuantityIri || *self == QUANTITY_LABEL
	}
}
