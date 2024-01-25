/// <https://schema.org/PropertyValueSpecification>
pub const PROPERTY_VALUE_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/PropertyValueSpecification";
/// <https://schema.org/PropertyValueSpecification>
pub const PROPERTY_VALUE_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/PropertyValueSpecification";
/// <https://schema.org/PropertyValueSpecification>
pub const PROPERTY_VALUE_SPECIFICATION_LABEL: &str = "PropertyValueSpecification";
pub struct PropertyValueSpecificationIri;
impl PartialEq<&str> for PropertyValueSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROPERTY_VALUE_SPECIFICATION_IRI_HTTP
			|| *other == PROPERTY_VALUE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<PropertyValueSpecificationIri> for &str {
	fn eq(&self, other: &PropertyValueSpecificationIri) -> bool {
		*self == PROPERTY_VALUE_SPECIFICATION_IRI_HTTP
			|| *self == PROPERTY_VALUE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct PropertyValueSpecificationIriOrLabel;
impl PartialEq<&str> for PropertyValueSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PropertyValueSpecificationIri || *other == PROPERTY_VALUE_SPECIFICATION_LABEL
	}
}
impl PartialEq<PropertyValueSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &PropertyValueSpecificationIriOrLabel) -> bool {
		*self == PropertyValueSpecificationIri || *self == PROPERTY_VALUE_SPECIFICATION_LABEL
	}
}
