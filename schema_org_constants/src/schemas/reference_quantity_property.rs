/// <https://schema.org/referenceQuantity>
pub const REFERENCE_QUANTITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/referenceQuantity";
/// <https://schema.org/referenceQuantity>
pub const REFERENCE_QUANTITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/referenceQuantity";
/// <https://schema.org/referenceQuantity>
pub const REFERENCE_QUANTITY_PROPERTY_LABEL: &str = "referenceQuantity";
pub struct ReferenceQuantityPropertyIri;
impl PartialEq<&str> for ReferenceQuantityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REFERENCE_QUANTITY_PROPERTY_IRI_HTTP
			|| *other == REFERENCE_QUANTITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReferenceQuantityPropertyIri> for &str {
	fn eq(&self, other: &ReferenceQuantityPropertyIri) -> bool {
		*self == REFERENCE_QUANTITY_PROPERTY_IRI_HTTP
			|| *self == REFERENCE_QUANTITY_PROPERTY_IRI_HTTPS
	}
}
pub struct ReferenceQuantityPropertyIriOrLabel;
impl PartialEq<&str> for ReferenceQuantityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReferenceQuantityPropertyIri || *other == REFERENCE_QUANTITY_PROPERTY_LABEL
	}
}
impl PartialEq<ReferenceQuantityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReferenceQuantityPropertyIriOrLabel) -> bool {
		*self == ReferenceQuantityPropertyIri || *self == REFERENCE_QUANTITY_PROPERTY_LABEL
	}
}
