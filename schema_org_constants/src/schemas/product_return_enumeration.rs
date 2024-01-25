/// <https://schema.org/ProductReturnEnumeration>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnEnumeration>."]
pub const PRODUCT_RETURN_ENUMERATION_IRI_HTTP: &str = "http://schema.org/ProductReturnEnumeration";
/// <https://schema.org/ProductReturnEnumeration>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnEnumeration>."]
pub const PRODUCT_RETURN_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/ProductReturnEnumeration";
/// <https://schema.org/ProductReturnEnumeration>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnEnumeration>."]
pub const PRODUCT_RETURN_ENUMERATION_LABEL: &str = "ProductReturnEnumeration";
pub struct ProductReturnEnumerationIri;
impl PartialEq<&str> for ProductReturnEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_RETURN_ENUMERATION_IRI_HTTP
			|| *other == PRODUCT_RETURN_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<ProductReturnEnumerationIri> for &str {
	fn eq(&self, other: &ProductReturnEnumerationIri) -> bool {
		*self == PRODUCT_RETURN_ENUMERATION_IRI_HTTP
			|| *self == PRODUCT_RETURN_ENUMERATION_IRI_HTTPS
	}
}
pub struct ProductReturnEnumerationIriOrLabel;
impl PartialEq<&str> for ProductReturnEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductReturnEnumerationIri || *other == PRODUCT_RETURN_ENUMERATION_LABEL
	}
}
impl PartialEq<ProductReturnEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &ProductReturnEnumerationIriOrLabel) -> bool {
		*self == ProductReturnEnumerationIri || *self == PRODUCT_RETURN_ENUMERATION_LABEL
	}
}
