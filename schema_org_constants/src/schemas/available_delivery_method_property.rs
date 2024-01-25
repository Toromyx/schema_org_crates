/// <https://schema.org/availableDeliveryMethod>
pub const AVAILABLE_DELIVERY_METHOD_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/availableDeliveryMethod";
/// <https://schema.org/availableDeliveryMethod>
pub const AVAILABLE_DELIVERY_METHOD_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/availableDeliveryMethod";
/// <https://schema.org/availableDeliveryMethod>
pub const AVAILABLE_DELIVERY_METHOD_PROPERTY_LABEL: &str = "availableDeliveryMethod";
pub struct AvailableDeliveryMethodPropertyIri;
impl PartialEq<&str> for AvailableDeliveryMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_DELIVERY_METHOD_PROPERTY_IRI_HTTP
			|| *other == AVAILABLE_DELIVERY_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableDeliveryMethodPropertyIri> for &str {
	fn eq(&self, other: &AvailableDeliveryMethodPropertyIri) -> bool {
		*self == AVAILABLE_DELIVERY_METHOD_PROPERTY_IRI_HTTP
			|| *self == AVAILABLE_DELIVERY_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableDeliveryMethodPropertyIriOrLabel;
impl PartialEq<&str> for AvailableDeliveryMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableDeliveryMethodPropertyIri
			|| *other == AVAILABLE_DELIVERY_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableDeliveryMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableDeliveryMethodPropertyIriOrLabel) -> bool {
		*self == AvailableDeliveryMethodPropertyIri
			|| *self == AVAILABLE_DELIVERY_METHOD_PROPERTY_LABEL
	}
}
