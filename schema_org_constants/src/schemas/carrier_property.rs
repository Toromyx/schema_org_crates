/// <https://schema.org/carrier>
#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
pub const CARRIER_PROPERTY_IRI_HTTP: &str = "http://schema.org/carrier";
/// <https://schema.org/carrier>
#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
pub const CARRIER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/carrier";
/// <https://schema.org/carrier>
#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
pub const CARRIER_PROPERTY_LABEL: &str = "carrier";
pub struct CarrierPropertyIri;
impl PartialEq<&str> for CarrierPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CARRIER_PROPERTY_IRI_HTTP || *other == CARRIER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CarrierPropertyIri> for &str {
	fn eq(&self, other: &CarrierPropertyIri) -> bool {
		*self == CARRIER_PROPERTY_IRI_HTTP || *self == CARRIER_PROPERTY_IRI_HTTPS
	}
}
pub struct CarrierPropertyIriOrLabel;
impl PartialEq<&str> for CarrierPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CarrierPropertyIri || *other == CARRIER_PROPERTY_LABEL
	}
}
impl PartialEq<CarrierPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CarrierPropertyIriOrLabel) -> bool {
		*self == CarrierPropertyIri || *self == CARRIER_PROPERTY_LABEL
	}
}
