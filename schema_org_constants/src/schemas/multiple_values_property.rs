/// <https://schema.org/multipleValues>
pub const MULTIPLE_VALUES_PROPERTY_IRI_HTTP: &str = "http://schema.org/multipleValues";
/// <https://schema.org/multipleValues>
pub const MULTIPLE_VALUES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/multipleValues";
/// <https://schema.org/multipleValues>
pub const MULTIPLE_VALUES_PROPERTY_LABEL: &str = "multipleValues";
pub struct MultipleValuesPropertyIri;
impl PartialEq<&str> for MultipleValuesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MULTIPLE_VALUES_PROPERTY_IRI_HTTP || *other == MULTIPLE_VALUES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MultipleValuesPropertyIri> for &str {
	fn eq(&self, other: &MultipleValuesPropertyIri) -> bool {
		*self == MULTIPLE_VALUES_PROPERTY_IRI_HTTP || *self == MULTIPLE_VALUES_PROPERTY_IRI_HTTPS
	}
}
pub struct MultipleValuesPropertyIriOrLabel;
impl PartialEq<&str> for MultipleValuesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MultipleValuesPropertyIri || *other == MULTIPLE_VALUES_PROPERTY_LABEL
	}
}
impl PartialEq<MultipleValuesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MultipleValuesPropertyIriOrLabel) -> bool {
		*self == MultipleValuesPropertyIri || *self == MULTIPLE_VALUES_PROPERTY_LABEL
	}
}
