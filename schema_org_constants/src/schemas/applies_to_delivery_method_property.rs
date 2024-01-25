/// <https://schema.org/appliesToDeliveryMethod>
pub const APPLIES_TO_DELIVERY_METHOD_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/appliesToDeliveryMethod";
/// <https://schema.org/appliesToDeliveryMethod>
pub const APPLIES_TO_DELIVERY_METHOD_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/appliesToDeliveryMethod";
/// <https://schema.org/appliesToDeliveryMethod>
pub const APPLIES_TO_DELIVERY_METHOD_PROPERTY_LABEL: &str = "appliesToDeliveryMethod";
pub struct AppliesToDeliveryMethodPropertyIri;
impl PartialEq<&str> for AppliesToDeliveryMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPLIES_TO_DELIVERY_METHOD_PROPERTY_IRI_HTTP
			|| *other == APPLIES_TO_DELIVERY_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AppliesToDeliveryMethodPropertyIri> for &str {
	fn eq(&self, other: &AppliesToDeliveryMethodPropertyIri) -> bool {
		*self == APPLIES_TO_DELIVERY_METHOD_PROPERTY_IRI_HTTP
			|| *self == APPLIES_TO_DELIVERY_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct AppliesToDeliveryMethodPropertyIriOrLabel;
impl PartialEq<&str> for AppliesToDeliveryMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AppliesToDeliveryMethodPropertyIri
			|| *other == APPLIES_TO_DELIVERY_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<AppliesToDeliveryMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AppliesToDeliveryMethodPropertyIriOrLabel) -> bool {
		*self == AppliesToDeliveryMethodPropertyIri
			|| *self == APPLIES_TO_DELIVERY_METHOD_PROPERTY_LABEL
	}
}
