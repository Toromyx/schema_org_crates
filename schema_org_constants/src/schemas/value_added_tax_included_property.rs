/// <https://schema.org/valueAddedTaxIncluded>
pub const VALUE_ADDED_TAX_INCLUDED_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/valueAddedTaxIncluded";
/// <https://schema.org/valueAddedTaxIncluded>
pub const VALUE_ADDED_TAX_INCLUDED_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/valueAddedTaxIncluded";
/// <https://schema.org/valueAddedTaxIncluded>
pub const VALUE_ADDED_TAX_INCLUDED_PROPERTY_LABEL: &str = "valueAddedTaxIncluded";
pub struct ValueAddedTaxIncludedPropertyIri;
impl PartialEq<&str> for ValueAddedTaxIncludedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALUE_ADDED_TAX_INCLUDED_PROPERTY_IRI_HTTP
			|| *other == VALUE_ADDED_TAX_INCLUDED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValueAddedTaxIncludedPropertyIri> for &str {
	fn eq(&self, other: &ValueAddedTaxIncludedPropertyIri) -> bool {
		*self == VALUE_ADDED_TAX_INCLUDED_PROPERTY_IRI_HTTP
			|| *self == VALUE_ADDED_TAX_INCLUDED_PROPERTY_IRI_HTTPS
	}
}
pub struct ValueAddedTaxIncludedPropertyIriOrLabel;
impl PartialEq<&str> for ValueAddedTaxIncludedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValueAddedTaxIncludedPropertyIri
			|| *other == VALUE_ADDED_TAX_INCLUDED_PROPERTY_LABEL
	}
}
impl PartialEq<ValueAddedTaxIncludedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValueAddedTaxIncludedPropertyIriOrLabel) -> bool {
		*self == ValueAddedTaxIncludedPropertyIri
			|| *self == VALUE_ADDED_TAX_INCLUDED_PROPERTY_LABEL
	}
}
