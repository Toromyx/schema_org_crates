/// <https://schema.org/checkoutPageURLTemplate>
pub const CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/checkoutPageURLTemplate";
/// <https://schema.org/checkoutPageURLTemplate>
pub const CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/checkoutPageURLTemplate";
/// <https://schema.org/checkoutPageURLTemplate>
pub const CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_LABEL: &str = "checkoutPageURLTemplate";
pub struct CheckoutPageUrlTemplatePropertyIri;
impl PartialEq<&str> for CheckoutPageUrlTemplatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_IRI_HTTP
			|| *other == CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CheckoutPageUrlTemplatePropertyIri> for &str {
	fn eq(&self, other: &CheckoutPageUrlTemplatePropertyIri) -> bool {
		*self == CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_IRI_HTTP
			|| *self == CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_IRI_HTTPS
	}
}
pub struct CheckoutPageUrlTemplatePropertyIriOrLabel;
impl PartialEq<&str> for CheckoutPageUrlTemplatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CheckoutPageUrlTemplatePropertyIri
			|| *other == CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_LABEL
	}
}
impl PartialEq<CheckoutPageUrlTemplatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CheckoutPageUrlTemplatePropertyIriOrLabel) -> bool {
		*self == CheckoutPageUrlTemplatePropertyIri
			|| *self == CHECKOUT_PAGE_URL_TEMPLATE_PROPERTY_LABEL
	}
}
