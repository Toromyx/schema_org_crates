/// <https://schema.org/busName>
pub const BUS_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/busName";
/// <https://schema.org/busName>
pub const BUS_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/busName";
/// <https://schema.org/busName>
pub const BUS_NAME_PROPERTY_LABEL: &str = "busName";
pub struct BusNamePropertyIri;
impl PartialEq<&str> for BusNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUS_NAME_PROPERTY_IRI_HTTP || *other == BUS_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BusNamePropertyIri> for &str {
	fn eq(&self, other: &BusNamePropertyIri) -> bool {
		*self == BUS_NAME_PROPERTY_IRI_HTTP || *self == BUS_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct BusNamePropertyIriOrLabel;
impl PartialEq<&str> for BusNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusNamePropertyIri || *other == BUS_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<BusNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BusNamePropertyIriOrLabel) -> bool {
		*self == BusNamePropertyIri || *self == BUS_NAME_PROPERTY_LABEL
	}
}
