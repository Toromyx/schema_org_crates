/// <https://schema.org/DeliveryChargeSpecification>
pub const DELIVERY_CHARGE_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/DeliveryChargeSpecification";
/// <https://schema.org/DeliveryChargeSpecification>
pub const DELIVERY_CHARGE_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/DeliveryChargeSpecification";
/// <https://schema.org/DeliveryChargeSpecification>
pub const DELIVERY_CHARGE_SPECIFICATION_LABEL: &str = "DeliveryChargeSpecification";
pub struct DeliveryChargeSpecificationIri;
impl PartialEq<&str> for DeliveryChargeSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DELIVERY_CHARGE_SPECIFICATION_IRI_HTTP
			|| *other == DELIVERY_CHARGE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<DeliveryChargeSpecificationIri> for &str {
	fn eq(&self, other: &DeliveryChargeSpecificationIri) -> bool {
		*self == DELIVERY_CHARGE_SPECIFICATION_IRI_HTTP
			|| *self == DELIVERY_CHARGE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct DeliveryChargeSpecificationIriOrLabel;
impl PartialEq<&str> for DeliveryChargeSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeliveryChargeSpecificationIri || *other == DELIVERY_CHARGE_SPECIFICATION_LABEL
	}
}
impl PartialEq<DeliveryChargeSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &DeliveryChargeSpecificationIriOrLabel) -> bool {
		*self == DeliveryChargeSpecificationIri || *self == DELIVERY_CHARGE_SPECIFICATION_LABEL
	}
}
