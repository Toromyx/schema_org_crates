/// <https://schema.org/broker>
pub const BROKER_PROPERTY_IRI_HTTP: &str = "http://schema.org/broker";
/// <https://schema.org/broker>
pub const BROKER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/broker";
/// <https://schema.org/broker>
pub const BROKER_PROPERTY_LABEL: &str = "broker";
pub struct BrokerPropertyIri;
impl PartialEq<&str> for BrokerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROKER_PROPERTY_IRI_HTTP || *other == BROKER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BrokerPropertyIri> for &str {
	fn eq(&self, other: &BrokerPropertyIri) -> bool {
		*self == BROKER_PROPERTY_IRI_HTTP || *self == BROKER_PROPERTY_IRI_HTTPS
	}
}
pub struct BrokerPropertyIriOrLabel;
impl PartialEq<&str> for BrokerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BrokerPropertyIri || *other == BROKER_PROPERTY_LABEL
	}
}
impl PartialEq<BrokerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BrokerPropertyIriOrLabel) -> bool {
		*self == BrokerPropertyIri || *self == BROKER_PROPERTY_LABEL
	}
}
