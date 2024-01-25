/// <https://schema.org/productReturnLink>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/merchantReturnLink>."]
pub const PRODUCT_RETURN_LINK_PROPERTY_IRI_HTTP: &str = "http://schema.org/productReturnLink";
/// <https://schema.org/productReturnLink>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/merchantReturnLink>."]
pub const PRODUCT_RETURN_LINK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/productReturnLink";
/// <https://schema.org/productReturnLink>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/merchantReturnLink>."]
pub const PRODUCT_RETURN_LINK_PROPERTY_LABEL: &str = "productReturnLink";
pub struct ProductReturnLinkPropertyIri;
impl PartialEq<&str> for ProductReturnLinkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_RETURN_LINK_PROPERTY_IRI_HTTP
			|| *other == PRODUCT_RETURN_LINK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProductReturnLinkPropertyIri> for &str {
	fn eq(&self, other: &ProductReturnLinkPropertyIri) -> bool {
		*self == PRODUCT_RETURN_LINK_PROPERTY_IRI_HTTP
			|| *self == PRODUCT_RETURN_LINK_PROPERTY_IRI_HTTPS
	}
}
pub struct ProductReturnLinkPropertyIriOrLabel;
impl PartialEq<&str> for ProductReturnLinkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductReturnLinkPropertyIri || *other == PRODUCT_RETURN_LINK_PROPERTY_LABEL
	}
}
impl PartialEq<ProductReturnLinkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProductReturnLinkPropertyIriOrLabel) -> bool {
		*self == ProductReturnLinkPropertyIri || *self == PRODUCT_RETURN_LINK_PROPERTY_LABEL
	}
}
