/// <https://schema.org/ProductReturnUnlimitedWindow>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnlimitedWindow>."]
pub const PRODUCT_RETURN_UNLIMITED_WINDOW_IRI_HTTP: &str =
	"http://schema.org/ProductReturnUnlimitedWindow";
/// <https://schema.org/ProductReturnUnlimitedWindow>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnlimitedWindow>."]
pub const PRODUCT_RETURN_UNLIMITED_WINDOW_IRI_HTTPS: &str =
	"https://schema.org/ProductReturnUnlimitedWindow";
/// <https://schema.org/ProductReturnUnlimitedWindow>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnUnlimitedWindow>."]
pub const PRODUCT_RETURN_UNLIMITED_WINDOW_LABEL: &str = "ProductReturnUnlimitedWindow";
pub struct ProductReturnUnlimitedWindowIri;
impl PartialEq<&str> for ProductReturnUnlimitedWindowIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_RETURN_UNLIMITED_WINDOW_IRI_HTTP
			|| *other == PRODUCT_RETURN_UNLIMITED_WINDOW_IRI_HTTPS
	}
}
impl PartialEq<ProductReturnUnlimitedWindowIri> for &str {
	fn eq(&self, other: &ProductReturnUnlimitedWindowIri) -> bool {
		*self == PRODUCT_RETURN_UNLIMITED_WINDOW_IRI_HTTP
			|| *self == PRODUCT_RETURN_UNLIMITED_WINDOW_IRI_HTTPS
	}
}
pub struct ProductReturnUnlimitedWindowIriOrLabel;
impl PartialEq<&str> for ProductReturnUnlimitedWindowIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductReturnUnlimitedWindowIri || *other == PRODUCT_RETURN_UNLIMITED_WINDOW_LABEL
	}
}
impl PartialEq<ProductReturnUnlimitedWindowIriOrLabel> for &str {
	fn eq(&self, other: &ProductReturnUnlimitedWindowIriOrLabel) -> bool {
		*self == ProductReturnUnlimitedWindowIri || *self == PRODUCT_RETURN_UNLIMITED_WINDOW_LABEL
	}
}
