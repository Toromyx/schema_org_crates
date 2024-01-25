/// <https://schema.org/ShippingRateSettings>
pub const SHIPPING_RATE_SETTINGS_IRI_HTTP: &str = "http://schema.org/ShippingRateSettings";
/// <https://schema.org/ShippingRateSettings>
pub const SHIPPING_RATE_SETTINGS_IRI_HTTPS: &str = "https://schema.org/ShippingRateSettings";
/// <https://schema.org/ShippingRateSettings>
pub const SHIPPING_RATE_SETTINGS_LABEL: &str = "ShippingRateSettings";
pub struct ShippingRateSettingsIri;
impl PartialEq<&str> for ShippingRateSettingsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHIPPING_RATE_SETTINGS_IRI_HTTP || *other == SHIPPING_RATE_SETTINGS_IRI_HTTPS
	}
}
impl PartialEq<ShippingRateSettingsIri> for &str {
	fn eq(&self, other: &ShippingRateSettingsIri) -> bool {
		*self == SHIPPING_RATE_SETTINGS_IRI_HTTP || *self == SHIPPING_RATE_SETTINGS_IRI_HTTPS
	}
}
pub struct ShippingRateSettingsIriOrLabel;
impl PartialEq<&str> for ShippingRateSettingsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShippingRateSettingsIri || *other == SHIPPING_RATE_SETTINGS_LABEL
	}
}
impl PartialEq<ShippingRateSettingsIriOrLabel> for &str {
	fn eq(&self, other: &ShippingRateSettingsIriOrLabel) -> bool {
		*self == ShippingRateSettingsIri || *self == SHIPPING_RATE_SETTINGS_LABEL
	}
}
