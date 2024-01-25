/// <https://schema.org/DeliveryTimeSettings>
pub const DELIVERY_TIME_SETTINGS_IRI_HTTP: &str = "http://schema.org/DeliveryTimeSettings";
/// <https://schema.org/DeliveryTimeSettings>
pub const DELIVERY_TIME_SETTINGS_IRI_HTTPS: &str = "https://schema.org/DeliveryTimeSettings";
/// <https://schema.org/DeliveryTimeSettings>
pub const DELIVERY_TIME_SETTINGS_LABEL: &str = "DeliveryTimeSettings";
pub struct DeliveryTimeSettingsIri;
impl PartialEq<&str> for DeliveryTimeSettingsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_TIME_SETTINGS_IRI_HTTP || *other == DELIVERY_TIME_SETTINGS_IRI_HTTPS
	}
}
impl PartialEq<DeliveryTimeSettingsIri> for &str {
	fn eq(&self, other: &DeliveryTimeSettingsIri) -> bool {
		*self == DELIVERY_TIME_SETTINGS_IRI_HTTP || *self == DELIVERY_TIME_SETTINGS_IRI_HTTPS
	}
}
pub struct DeliveryTimeSettingsIriOrLabel;
impl PartialEq<&str> for DeliveryTimeSettingsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryTimeSettingsIri || *other == DELIVERY_TIME_SETTINGS_LABEL
	}
}
impl PartialEq<DeliveryTimeSettingsIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryTimeSettingsIriOrLabel) -> bool {
		*self == DeliveryTimeSettingsIri || *self == DELIVERY_TIME_SETTINGS_LABEL
	}
}
