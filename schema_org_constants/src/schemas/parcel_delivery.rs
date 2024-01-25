/// <https://schema.org/ParcelDelivery>
pub const PARCEL_DELIVERY_IRI_HTTP: &str = "http://schema.org/ParcelDelivery";
/// <https://schema.org/ParcelDelivery>
pub const PARCEL_DELIVERY_IRI_HTTPS: &str = "https://schema.org/ParcelDelivery";
/// <https://schema.org/ParcelDelivery>
pub const PARCEL_DELIVERY_LABEL: &str = "ParcelDelivery";
pub struct ParcelDeliveryIri;
impl PartialEq<&str> for ParcelDeliveryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARCEL_DELIVERY_IRI_HTTP || *other == PARCEL_DELIVERY_IRI_HTTPS
	}
}
impl PartialEq<ParcelDeliveryIri> for &str {
	fn eq(&self, other: &ParcelDeliveryIri) -> bool {
		*self == PARCEL_DELIVERY_IRI_HTTP || *self == PARCEL_DELIVERY_IRI_HTTPS
	}
}
pub struct ParcelDeliveryIriOrLabel;
impl PartialEq<&str> for ParcelDeliveryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParcelDeliveryIri || *other == PARCEL_DELIVERY_LABEL
	}
}
impl PartialEq<ParcelDeliveryIriOrLabel> for &str {
	fn eq(&self, other: &ParcelDeliveryIriOrLabel) -> bool {
		*self == ParcelDeliveryIri || *self == PARCEL_DELIVERY_LABEL
	}
}
