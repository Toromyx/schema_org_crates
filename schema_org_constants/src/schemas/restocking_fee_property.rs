/// <https://schema.org/restockingFee>
pub const RESTOCKING_FEE_PROPERTY_IRI_HTTP: &str = "http://schema.org/restockingFee";
/// <https://schema.org/restockingFee>
pub const RESTOCKING_FEE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/restockingFee";
/// <https://schema.org/restockingFee>
pub const RESTOCKING_FEE_PROPERTY_LABEL: &str = "restockingFee";
pub struct RestockingFeePropertyIri;
impl PartialEq<&str> for RestockingFeePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESTOCKING_FEE_PROPERTY_IRI_HTTP || *other == RESTOCKING_FEE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RestockingFeePropertyIri> for &str {
	fn eq(&self, other: &RestockingFeePropertyIri) -> bool {
		*self == RESTOCKING_FEE_PROPERTY_IRI_HTTP || *self == RESTOCKING_FEE_PROPERTY_IRI_HTTPS
	}
}
pub struct RestockingFeePropertyIriOrLabel;
impl PartialEq<&str> for RestockingFeePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RestockingFeePropertyIri || *other == RESTOCKING_FEE_PROPERTY_LABEL
	}
}
impl PartialEq<RestockingFeePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RestockingFeePropertyIriOrLabel) -> bool {
		*self == RestockingFeePropertyIri || *self == RESTOCKING_FEE_PROPERTY_LABEL
	}
}
