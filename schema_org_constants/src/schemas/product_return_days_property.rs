/// <https://schema.org/productReturnDays>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/merchantReturnDays>."]
pub const PRODUCT_RETURN_DAYS_PROPERTY_IRI_HTTP: &str = "http://schema.org/productReturnDays";
/// <https://schema.org/productReturnDays>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/merchantReturnDays>."]
pub const PRODUCT_RETURN_DAYS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/productReturnDays";
/// <https://schema.org/productReturnDays>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>. This schema is superseded by <https://schema.org/merchantReturnDays>."]
pub const PRODUCT_RETURN_DAYS_PROPERTY_LABEL: &str = "productReturnDays";
pub struct ProductReturnDaysPropertyIri;
impl PartialEq<&str> for ProductReturnDaysPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_RETURN_DAYS_PROPERTY_IRI_HTTP
			|| *other == PRODUCT_RETURN_DAYS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProductReturnDaysPropertyIri> for &str {
	fn eq(&self, other: &ProductReturnDaysPropertyIri) -> bool {
		*self == PRODUCT_RETURN_DAYS_PROPERTY_IRI_HTTP
			|| *self == PRODUCT_RETURN_DAYS_PROPERTY_IRI_HTTPS
	}
}
pub struct ProductReturnDaysPropertyIriOrLabel;
impl PartialEq<&str> for ProductReturnDaysPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductReturnDaysPropertyIri || *other == PRODUCT_RETURN_DAYS_PROPERTY_LABEL
	}
}
impl PartialEq<ProductReturnDaysPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProductReturnDaysPropertyIriOrLabel) -> bool {
		*self == ProductReturnDaysPropertyIri || *self == PRODUCT_RETURN_DAYS_PROPERTY_LABEL
	}
}
