/// <https://schema.org/LockerDelivery>
pub const LOCKER_DELIVERY_IRI_HTTP: &str = "http://schema.org/LockerDelivery";
/// <https://schema.org/LockerDelivery>
pub const LOCKER_DELIVERY_IRI_HTTPS: &str = "https://schema.org/LockerDelivery";
/// <https://schema.org/LockerDelivery>
pub const LOCKER_DELIVERY_LABEL: &str = "LockerDelivery";
pub struct LockerDeliveryIri;
impl PartialEq<&str> for LockerDeliveryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOCKER_DELIVERY_IRI_HTTP || *other == LOCKER_DELIVERY_IRI_HTTPS
	}
}
impl PartialEq<LockerDeliveryIri> for &str {
	fn eq(&self, other: &LockerDeliveryIri) -> bool {
		*self == LOCKER_DELIVERY_IRI_HTTP || *self == LOCKER_DELIVERY_IRI_HTTPS
	}
}
pub struct LockerDeliveryIriOrLabel;
impl PartialEq<&str> for LockerDeliveryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LockerDeliveryIri || *other == LOCKER_DELIVERY_LABEL
	}
}
impl PartialEq<LockerDeliveryIriOrLabel> for &str {
	fn eq(&self, other: &LockerDeliveryIriOrLabel) -> bool {
		*self == LockerDeliveryIri || *self == LOCKER_DELIVERY_LABEL
	}
}
