/// <https://schema.org/ProductReturnPolicy>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnPolicy>."]
pub const PRODUCT_RETURN_POLICY_IRI_HTTP: &str = "http://schema.org/ProductReturnPolicy";
/// <https://schema.org/ProductReturnPolicy>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnPolicy>."]
pub const PRODUCT_RETURN_POLICY_IRI_HTTPS: &str = "https://schema.org/ProductReturnPolicy";
/// <https://schema.org/ProductReturnPolicy>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/MerchantReturnPolicy>."]
pub const PRODUCT_RETURN_POLICY_LABEL: &str = "ProductReturnPolicy";
pub struct ProductReturnPolicyIri;
impl PartialEq<&str> for ProductReturnPolicyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_RETURN_POLICY_IRI_HTTP || *other == PRODUCT_RETURN_POLICY_IRI_HTTPS
	}
}
impl PartialEq<ProductReturnPolicyIri> for &str {
	fn eq(&self, other: &ProductReturnPolicyIri) -> bool {
		*self == PRODUCT_RETURN_POLICY_IRI_HTTP || *self == PRODUCT_RETURN_POLICY_IRI_HTTPS
	}
}
pub struct ProductReturnPolicyIriOrLabel;
impl PartialEq<&str> for ProductReturnPolicyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductReturnPolicyIri || *other == PRODUCT_RETURN_POLICY_LABEL
	}
}
impl PartialEq<ProductReturnPolicyIriOrLabel> for &str {
	fn eq(&self, other: &ProductReturnPolicyIriOrLabel) -> bool {
		*self == ProductReturnPolicyIri || *self == PRODUCT_RETURN_POLICY_LABEL
	}
}
