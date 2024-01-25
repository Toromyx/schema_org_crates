/// <https://schema.org/ProductReturnUnspecified>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnspecified>."]
pub const PRODUCT_RETURN_UNSPECIFIED_IRI_HTTP: &str = "http://schema.org/ProductReturnUnspecified";
/// <https://schema.org/ProductReturnUnspecified>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnspecified>."]
pub const PRODUCT_RETURN_UNSPECIFIED_IRI_HTTPS: &str =
	"https://schema.org/ProductReturnUnspecified";
/// <https://schema.org/ProductReturnUnspecified>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnspecified>."]
pub const PRODUCT_RETURN_UNSPECIFIED_LABEL: &str = "ProductReturnUnspecified";
pub struct ProductReturnUnspecifiedIri;
impl PartialEq<&str> for ProductReturnUnspecifiedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_RETURN_UNSPECIFIED_IRI_HTTP
			|| *other == PRODUCT_RETURN_UNSPECIFIED_IRI_HTTPS
	}
}
impl PartialEq<ProductReturnUnspecifiedIri> for &str {
	fn eq(&self, other: &ProductReturnUnspecifiedIri) -> bool {
		*self == PRODUCT_RETURN_UNSPECIFIED_IRI_HTTP
			|| *self == PRODUCT_RETURN_UNSPECIFIED_IRI_HTTPS
	}
}
pub struct ProductReturnUnspecifiedIriOrLabel;
impl PartialEq<&str> for ProductReturnUnspecifiedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductReturnUnspecifiedIri || *other == PRODUCT_RETURN_UNSPECIFIED_LABEL
	}
}
impl PartialEq<ProductReturnUnspecifiedIriOrLabel> for &str {
	fn eq(&self, other: &ProductReturnUnspecifiedIriOrLabel) -> bool {
		*self == ProductReturnUnspecifiedIri || *self == PRODUCT_RETURN_UNSPECIFIED_LABEL
	}
}
