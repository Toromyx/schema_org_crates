/// <https://schema.org/freeShippingThreshold>
pub const FREE_SHIPPING_THRESHOLD_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/freeShippingThreshold";
/// <https://schema.org/freeShippingThreshold>
pub const FREE_SHIPPING_THRESHOLD_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/freeShippingThreshold";
/// <https://schema.org/freeShippingThreshold>
pub const FREE_SHIPPING_THRESHOLD_PROPERTY_LABEL: &str = "freeShippingThreshold";
pub struct FreeShippingThresholdPropertyIri;
impl PartialEq<&str> for FreeShippingThresholdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FREE_SHIPPING_THRESHOLD_PROPERTY_IRI_HTTP
			|| *other == FREE_SHIPPING_THRESHOLD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FreeShippingThresholdPropertyIri> for &str {
	fn eq(&self, other: &FreeShippingThresholdPropertyIri) -> bool {
		*self == FREE_SHIPPING_THRESHOLD_PROPERTY_IRI_HTTP
			|| *self == FREE_SHIPPING_THRESHOLD_PROPERTY_IRI_HTTPS
	}
}
pub struct FreeShippingThresholdPropertyIriOrLabel;
impl PartialEq<&str> for FreeShippingThresholdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FreeShippingThresholdPropertyIri
			|| *other == FREE_SHIPPING_THRESHOLD_PROPERTY_LABEL
	}
}
impl PartialEq<FreeShippingThresholdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FreeShippingThresholdPropertyIriOrLabel) -> bool {
		*self == FreeShippingThresholdPropertyIri || *self == FREE_SHIPPING_THRESHOLD_PROPERTY_LABEL
	}
}
