/// <https://schema.org/CheckoutPage>
pub const CHECKOUT_PAGE_IRI_HTTP: &str = "http://schema.org/CheckoutPage";
/// <https://schema.org/CheckoutPage>
pub const CHECKOUT_PAGE_IRI_HTTPS: &str = "https://schema.org/CheckoutPage";
/// <https://schema.org/CheckoutPage>
pub const CHECKOUT_PAGE_LABEL: &str = "CheckoutPage";
pub struct CheckoutPageIri;
impl PartialEq<&str> for CheckoutPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHECKOUT_PAGE_IRI_HTTP || *other == CHECKOUT_PAGE_IRI_HTTPS
	}
}
impl PartialEq<CheckoutPageIri> for &str {
	fn eq(&self, other: &CheckoutPageIri) -> bool {
		*self == CHECKOUT_PAGE_IRI_HTTP || *self == CHECKOUT_PAGE_IRI_HTTPS
	}
}
pub struct CheckoutPageIriOrLabel;
impl PartialEq<&str> for CheckoutPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CheckoutPageIri || *other == CHECKOUT_PAGE_LABEL
	}
}
impl PartialEq<CheckoutPageIriOrLabel> for &str {
	fn eq(&self, other: &CheckoutPageIriOrLabel) -> bool {
		*self == CheckoutPageIri || *self == CHECKOUT_PAGE_LABEL
	}
}
