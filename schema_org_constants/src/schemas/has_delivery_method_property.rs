/// <https://schema.org/hasDeliveryMethod>
pub const HAS_DELIVERY_METHOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasDeliveryMethod";
/// <https://schema.org/hasDeliveryMethod>
pub const HAS_DELIVERY_METHOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasDeliveryMethod";
/// <https://schema.org/hasDeliveryMethod>
pub const HAS_DELIVERY_METHOD_PROPERTY_LABEL: &str = "hasDeliveryMethod";
pub struct HasDeliveryMethodPropertyIri;
impl PartialEq<&str> for HasDeliveryMethodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_DELIVERY_METHOD_PROPERTY_IRI_HTTP
			|| *other == HAS_DELIVERY_METHOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasDeliveryMethodPropertyIri> for &str {
	fn eq(&self, other: &HasDeliveryMethodPropertyIri) -> bool {
		*self == HAS_DELIVERY_METHOD_PROPERTY_IRI_HTTP
			|| *self == HAS_DELIVERY_METHOD_PROPERTY_IRI_HTTPS
	}
}
pub struct HasDeliveryMethodPropertyIriOrLabel;
impl PartialEq<&str> for HasDeliveryMethodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasDeliveryMethodPropertyIri || *other == HAS_DELIVERY_METHOD_PROPERTY_LABEL
	}
}
impl PartialEq<HasDeliveryMethodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasDeliveryMethodPropertyIriOrLabel) -> bool {
		*self == HasDeliveryMethodPropertyIri || *self == HAS_DELIVERY_METHOD_PROPERTY_LABEL
	}
}
