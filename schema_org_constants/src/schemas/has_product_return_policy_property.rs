/// <https://schema.org/hasProductReturnPolicy>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/hasMerchantReturnPolicy>."]
pub const HAS_PRODUCT_RETURN_POLICY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasProductReturnPolicy";
/// <https://schema.org/hasProductReturnPolicy>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/hasMerchantReturnPolicy>."]
pub const HAS_PRODUCT_RETURN_POLICY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasProductReturnPolicy";
/// <https://schema.org/hasProductReturnPolicy>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/hasMerchantReturnPolicy>."]
pub const HAS_PRODUCT_RETURN_POLICY_PROPERTY_LABEL: &str = "hasProductReturnPolicy";
pub struct HasProductReturnPolicyPropertyIri;
impl PartialEq<&str> for HasProductReturnPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_PRODUCT_RETURN_POLICY_PROPERTY_IRI_HTTP
			|| *other == HAS_PRODUCT_RETURN_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasProductReturnPolicyPropertyIri> for &str {
	fn eq(&self, other: &HasProductReturnPolicyPropertyIri) -> bool {
		*self == HAS_PRODUCT_RETURN_POLICY_PROPERTY_IRI_HTTP
			|| *self == HAS_PRODUCT_RETURN_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct HasProductReturnPolicyPropertyIriOrLabel;
impl PartialEq<&str> for HasProductReturnPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasProductReturnPolicyPropertyIri
			|| *other == HAS_PRODUCT_RETURN_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<HasProductReturnPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasProductReturnPolicyPropertyIriOrLabel) -> bool {
		*self == HasProductReturnPolicyPropertyIri
			|| *self == HAS_PRODUCT_RETURN_POLICY_PROPERTY_LABEL
	}
}
