/// <https://schema.org/ProductReturnFiniteReturnWindow>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnFiniteReturnWindow>."]
pub const PRODUCT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTP: &str =
	"http://schema.org/ProductReturnFiniteReturnWindow";
/// <https://schema.org/ProductReturnFiniteReturnWindow>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnFiniteReturnWindow>."]
pub const PRODUCT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTPS: &str =
	"https://schema.org/ProductReturnFiniteReturnWindow";
/// <https://schema.org/ProductReturnFiniteReturnWindow>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnFiniteReturnWindow>."]
pub const PRODUCT_RETURN_FINITE_RETURN_WINDOW_LABEL: &str = "ProductReturnFiniteReturnWindow";
pub struct ProductReturnFiniteReturnWindowIri;
impl PartialEq<&str> for ProductReturnFiniteReturnWindowIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTP
			|| *other == PRODUCT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTPS
	}
}
impl PartialEq<ProductReturnFiniteReturnWindowIri> for &str {
	fn eq(&self, other: &ProductReturnFiniteReturnWindowIri) -> bool {
		*self == PRODUCT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTP
			|| *self == PRODUCT_RETURN_FINITE_RETURN_WINDOW_IRI_HTTPS
	}
}
pub struct ProductReturnFiniteReturnWindowIriOrLabel;
impl PartialEq<&str> for ProductReturnFiniteReturnWindowIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductReturnFiniteReturnWindowIri
			|| *other == PRODUCT_RETURN_FINITE_RETURN_WINDOW_LABEL
	}
}
impl PartialEq<ProductReturnFiniteReturnWindowIriOrLabel> for &str {
	fn eq(&self, other: &ProductReturnFiniteReturnWindowIriOrLabel) -> bool {
		*self == ProductReturnFiniteReturnWindowIri
			|| *self == PRODUCT_RETURN_FINITE_RETURN_WINDOW_LABEL
	}
}
