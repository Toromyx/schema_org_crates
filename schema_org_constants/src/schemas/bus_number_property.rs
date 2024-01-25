/// <https://schema.org/busNumber>
pub const BUS_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/busNumber";
/// <https://schema.org/busNumber>
pub const BUS_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/busNumber";
/// <https://schema.org/busNumber>
pub const BUS_NUMBER_PROPERTY_LABEL: &str = "busNumber";
pub struct BusNumberPropertyIri;
impl PartialEq<&str> for BusNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUS_NUMBER_PROPERTY_IRI_HTTP || *other == BUS_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BusNumberPropertyIri> for &str {
	fn eq(&self, other: &BusNumberPropertyIri) -> bool {
		*self == BUS_NUMBER_PROPERTY_IRI_HTTP || *self == BUS_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct BusNumberPropertyIriOrLabel;
impl PartialEq<&str> for BusNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusNumberPropertyIri || *other == BUS_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<BusNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BusNumberPropertyIriOrLabel) -> bool {
		*self == BusNumberPropertyIri || *self == BUS_NUMBER_PROPERTY_LABEL
	}
}
