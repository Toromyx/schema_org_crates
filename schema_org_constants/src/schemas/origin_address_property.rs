/// <https://schema.org/originAddress>
pub const ORIGIN_ADDRESS_PROPERTY_IRI_HTTP: &str = "http://schema.org/originAddress";
/// <https://schema.org/originAddress>
pub const ORIGIN_ADDRESS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/originAddress";
/// <https://schema.org/originAddress>
pub const ORIGIN_ADDRESS_PROPERTY_LABEL: &str = "originAddress";
pub struct OriginAddressPropertyIri;
impl PartialEq<&str> for OriginAddressPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORIGIN_ADDRESS_PROPERTY_IRI_HTTP || *other == ORIGIN_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OriginAddressPropertyIri> for &str {
	fn eq(&self, other: &OriginAddressPropertyIri) -> bool {
		*self == ORIGIN_ADDRESS_PROPERTY_IRI_HTTP || *self == ORIGIN_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
pub struct OriginAddressPropertyIriOrLabel;
impl PartialEq<&str> for OriginAddressPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OriginAddressPropertyIri || *other == ORIGIN_ADDRESS_PROPERTY_LABEL
	}
}
impl PartialEq<OriginAddressPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OriginAddressPropertyIriOrLabel) -> bool {
		*self == OriginAddressPropertyIri || *self == ORIGIN_ADDRESS_PROPERTY_LABEL
	}
}
