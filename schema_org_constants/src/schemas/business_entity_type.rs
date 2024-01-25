/// <https://schema.org/BusinessEntityType>
pub const BUSINESS_ENTITY_TYPE_IRI_HTTP: &str = "http://schema.org/BusinessEntityType";
/// <https://schema.org/BusinessEntityType>
pub const BUSINESS_ENTITY_TYPE_IRI_HTTPS: &str = "https://schema.org/BusinessEntityType";
/// <https://schema.org/BusinessEntityType>
pub const BUSINESS_ENTITY_TYPE_LABEL: &str = "BusinessEntityType";
pub struct BusinessEntityTypeIri;
impl PartialEq<&str> for BusinessEntityTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUSINESS_ENTITY_TYPE_IRI_HTTP || *other == BUSINESS_ENTITY_TYPE_IRI_HTTPS
	}
}
impl PartialEq<BusinessEntityTypeIri> for &str {
	fn eq(&self, other: &BusinessEntityTypeIri) -> bool {
		*self == BUSINESS_ENTITY_TYPE_IRI_HTTP || *self == BUSINESS_ENTITY_TYPE_IRI_HTTPS
	}
}
pub struct BusinessEntityTypeIriOrLabel;
impl PartialEq<&str> for BusinessEntityTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusinessEntityTypeIri || *other == BUSINESS_ENTITY_TYPE_LABEL
	}
}
impl PartialEq<BusinessEntityTypeIriOrLabel> for &str {
	fn eq(&self, other: &BusinessEntityTypeIriOrLabel) -> bool {
		*self == BusinessEntityTypeIri || *self == BUSINESS_ENTITY_TYPE_LABEL
	}
}
