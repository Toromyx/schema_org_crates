/// <https://schema.org/QuantitativeValue>
pub const QUANTITATIVE_VALUE_IRI_HTTP: &str = "http://schema.org/QuantitativeValue";
/// <https://schema.org/QuantitativeValue>
pub const QUANTITATIVE_VALUE_IRI_HTTPS: &str = "https://schema.org/QuantitativeValue";
/// <https://schema.org/QuantitativeValue>
pub const QUANTITATIVE_VALUE_LABEL: &str = "QuantitativeValue";
pub struct QuantitativeValueIri;
impl PartialEq<&str> for QuantitativeValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUANTITATIVE_VALUE_IRI_HTTP || *other == QUANTITATIVE_VALUE_IRI_HTTPS
	}
}
impl PartialEq<QuantitativeValueIri> for &str {
	fn eq(&self, other: &QuantitativeValueIri) -> bool {
		*self == QUANTITATIVE_VALUE_IRI_HTTP || *self == QUANTITATIVE_VALUE_IRI_HTTPS
	}
}
pub struct QuantitativeValueIriOrLabel;
impl PartialEq<&str> for QuantitativeValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuantitativeValueIri || *other == QUANTITATIVE_VALUE_LABEL
	}
}
impl PartialEq<QuantitativeValueIriOrLabel> for &str {
	fn eq(&self, other: &QuantitativeValueIriOrLabel) -> bool {
		*self == QuantitativeValueIri || *self == QUANTITATIVE_VALUE_LABEL
	}
}
