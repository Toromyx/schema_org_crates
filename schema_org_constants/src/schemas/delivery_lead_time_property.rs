/// <https://schema.org/deliveryLeadTime>
pub const DELIVERY_LEAD_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/deliveryLeadTime";
/// <https://schema.org/deliveryLeadTime>
pub const DELIVERY_LEAD_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/deliveryLeadTime";
/// <https://schema.org/deliveryLeadTime>
pub const DELIVERY_LEAD_TIME_PROPERTY_LABEL: &str = "deliveryLeadTime";
pub struct DeliveryLeadTimePropertyIri;
impl PartialEq<&str> for DeliveryLeadTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_LEAD_TIME_PROPERTY_IRI_HTTP
			|| *other == DELIVERY_LEAD_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DeliveryLeadTimePropertyIri> for &str {
	fn eq(&self, other: &DeliveryLeadTimePropertyIri) -> bool {
		*self == DELIVERY_LEAD_TIME_PROPERTY_IRI_HTTP
			|| *self == DELIVERY_LEAD_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct DeliveryLeadTimePropertyIriOrLabel;
impl PartialEq<&str> for DeliveryLeadTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryLeadTimePropertyIri || *other == DELIVERY_LEAD_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<DeliveryLeadTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryLeadTimePropertyIriOrLabel) -> bool {
		*self == DeliveryLeadTimePropertyIri || *self == DELIVERY_LEAD_TIME_PROPERTY_LABEL
	}
}
