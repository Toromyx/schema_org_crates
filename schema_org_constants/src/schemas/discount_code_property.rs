/// <https://schema.org/discountCode>
pub const DISCOUNT_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/discountCode";
/// <https://schema.org/discountCode>
pub const DISCOUNT_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/discountCode";
/// <https://schema.org/discountCode>
pub const DISCOUNT_CODE_PROPERTY_LABEL: &str = "discountCode";
pub struct DiscountCodePropertyIri;
impl PartialEq<&str> for DiscountCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISCOUNT_CODE_PROPERTY_IRI_HTTP || *other == DISCOUNT_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiscountCodePropertyIri> for &str {
	fn eq(&self, other: &DiscountCodePropertyIri) -> bool {
		*self == DISCOUNT_CODE_PROPERTY_IRI_HTTP || *self == DISCOUNT_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct DiscountCodePropertyIriOrLabel;
impl PartialEq<&str> for DiscountCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiscountCodePropertyIri || *other == DISCOUNT_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<DiscountCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiscountCodePropertyIriOrLabel) -> bool {
		*self == DiscountCodePropertyIri || *self == DISCOUNT_CODE_PROPERTY_LABEL
	}
}
