/// <https://schema.org/discount>
pub const DISCOUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/discount";
/// <https://schema.org/discount>
pub const DISCOUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/discount";
/// <https://schema.org/discount>
pub const DISCOUNT_PROPERTY_LABEL: &str = "discount";
pub struct DiscountPropertyIri;
impl PartialEq<&str> for DiscountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISCOUNT_PROPERTY_IRI_HTTP || *other == DISCOUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiscountPropertyIri> for &str {
	fn eq(&self, other: &DiscountPropertyIri) -> bool {
		*self == DISCOUNT_PROPERTY_IRI_HTTP || *self == DISCOUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct DiscountPropertyIriOrLabel;
impl PartialEq<&str> for DiscountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiscountPropertyIri || *other == DISCOUNT_PROPERTY_LABEL
	}
}
impl PartialEq<DiscountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiscountPropertyIriOrLabel) -> bool {
		*self == DiscountPropertyIri || *self == DISCOUNT_PROPERTY_LABEL
	}
}
