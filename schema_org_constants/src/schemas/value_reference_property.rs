/// <https://schema.org/valueReference>
pub const VALUE_REFERENCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/valueReference";
/// <https://schema.org/valueReference>
pub const VALUE_REFERENCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/valueReference";
/// <https://schema.org/valueReference>
pub const VALUE_REFERENCE_PROPERTY_LABEL: &str = "valueReference";
pub struct ValueReferencePropertyIri;
impl PartialEq<&str> for ValueReferencePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VALUE_REFERENCE_PROPERTY_IRI_HTTP || *other == VALUE_REFERENCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ValueReferencePropertyIri> for &str {
	fn eq(&self, other: &ValueReferencePropertyIri) -> bool {
		*self == VALUE_REFERENCE_PROPERTY_IRI_HTTP || *self == VALUE_REFERENCE_PROPERTY_IRI_HTTPS
	}
}
pub struct ValueReferencePropertyIriOrLabel;
impl PartialEq<&str> for ValueReferencePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ValueReferencePropertyIri || *other == VALUE_REFERENCE_PROPERTY_LABEL
	}
}
impl PartialEq<ValueReferencePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ValueReferencePropertyIriOrLabel) -> bool {
		*self == ValueReferencePropertyIri || *self == VALUE_REFERENCE_PROPERTY_LABEL
	}
}
