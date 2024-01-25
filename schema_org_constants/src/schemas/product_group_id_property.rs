/// <https://schema.org/productGroupID>
pub const PRODUCT_GROUP_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/productGroupID";
/// <https://schema.org/productGroupID>
pub const PRODUCT_GROUP_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/productGroupID";
/// <https://schema.org/productGroupID>
pub const PRODUCT_GROUP_ID_PROPERTY_LABEL: &str = "productGroupID";
pub struct ProductGroupIdPropertyIri;
impl PartialEq<&str> for ProductGroupIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_GROUP_ID_PROPERTY_IRI_HTTP
			|| *other == PRODUCT_GROUP_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProductGroupIdPropertyIri> for &str {
	fn eq(&self, other: &ProductGroupIdPropertyIri) -> bool {
		*self == PRODUCT_GROUP_ID_PROPERTY_IRI_HTTP || *self == PRODUCT_GROUP_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct ProductGroupIdPropertyIriOrLabel;
impl PartialEq<&str> for ProductGroupIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductGroupIdPropertyIri || *other == PRODUCT_GROUP_ID_PROPERTY_LABEL
	}
}
impl PartialEq<ProductGroupIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProductGroupIdPropertyIriOrLabel) -> bool {
		*self == ProductGroupIdPropertyIri || *self == PRODUCT_GROUP_ID_PROPERTY_LABEL
	}
}
