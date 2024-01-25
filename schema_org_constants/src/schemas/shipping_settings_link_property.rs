/// <https://schema.org/shippingSettingsLink>
pub const SHIPPING_SETTINGS_LINK_PROPERTY_IRI_HTTP: &str = "http://schema.org/shippingSettingsLink";
/// <https://schema.org/shippingSettingsLink>
pub const SHIPPING_SETTINGS_LINK_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/shippingSettingsLink";
/// <https://schema.org/shippingSettingsLink>
pub const SHIPPING_SETTINGS_LINK_PROPERTY_LABEL: &str = "shippingSettingsLink";
pub struct ShippingSettingsLinkPropertyIri;
impl PartialEq<&str> for ShippingSettingsLinkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHIPPING_SETTINGS_LINK_PROPERTY_IRI_HTTP
			|| *other == SHIPPING_SETTINGS_LINK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ShippingSettingsLinkPropertyIri> for &str {
	fn eq(&self, other: &ShippingSettingsLinkPropertyIri) -> bool {
		*self == SHIPPING_SETTINGS_LINK_PROPERTY_IRI_HTTP
			|| *self == SHIPPING_SETTINGS_LINK_PROPERTY_IRI_HTTPS
	}
}
pub struct ShippingSettingsLinkPropertyIriOrLabel;
impl PartialEq<&str> for ShippingSettingsLinkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShippingSettingsLinkPropertyIri || *other == SHIPPING_SETTINGS_LINK_PROPERTY_LABEL
	}
}
impl PartialEq<ShippingSettingsLinkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ShippingSettingsLinkPropertyIriOrLabel) -> bool {
		*self == ShippingSettingsLinkPropertyIri || *self == SHIPPING_SETTINGS_LINK_PROPERTY_LABEL
	}
}
