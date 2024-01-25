/// <https://schema.org/ProductReturnNotPermitted>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnNotPermitted>."]
pub const PRODUCT_RETURN_NOT_PERMITTED_IRI_HTTP: &str =
	"http://schema.org/ProductReturnNotPermitted";
/// <https://schema.org/ProductReturnNotPermitted>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnNotPermitted>."]
pub const PRODUCT_RETURN_NOT_PERMITTED_IRI_HTTPS: &str =
	"https://schema.org/ProductReturnNotPermitted";
/// <https://schema.org/ProductReturnNotPermitted>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnNotPermitted>."]
pub const PRODUCT_RETURN_NOT_PERMITTED_LABEL: &str = "ProductReturnNotPermitted";
pub struct ProductReturnNotPermittedIri;
impl PartialEq<&str> for ProductReturnNotPermittedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_RETURN_NOT_PERMITTED_IRI_HTTP
			|| *other == PRODUCT_RETURN_NOT_PERMITTED_IRI_HTTPS
	}
}
impl PartialEq<ProductReturnNotPermittedIri> for &str {
	fn eq(&self, other: &ProductReturnNotPermittedIri) -> bool {
		*self == PRODUCT_RETURN_NOT_PERMITTED_IRI_HTTP
			|| *self == PRODUCT_RETURN_NOT_PERMITTED_IRI_HTTPS
	}
}
pub struct ProductReturnNotPermittedIriOrLabel;
impl PartialEq<&str> for ProductReturnNotPermittedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductReturnNotPermittedIri || *other == PRODUCT_RETURN_NOT_PERMITTED_LABEL
	}
}
impl PartialEq<ProductReturnNotPermittedIriOrLabel> for &str {
	fn eq(&self, other: &ProductReturnNotPermittedIriOrLabel) -> bool {
		*self == ProductReturnNotPermittedIri || *self == PRODUCT_RETURN_NOT_PERMITTED_LABEL
	}
}
