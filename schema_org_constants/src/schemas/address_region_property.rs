/// <https://schema.org/addressRegion>
pub const ADDRESS_REGION_PROPERTY_IRI_HTTP: &str = "http://schema.org/addressRegion";
/// <https://schema.org/addressRegion>
pub const ADDRESS_REGION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/addressRegion";
/// <https://schema.org/addressRegion>
pub const ADDRESS_REGION_PROPERTY_LABEL: &str = "addressRegion";
pub struct AddressRegionPropertyIri;
impl PartialEq<&str> for AddressRegionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDRESS_REGION_PROPERTY_IRI_HTTP || *other == ADDRESS_REGION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AddressRegionPropertyIri> for &str {
	fn eq(&self, other: &AddressRegionPropertyIri) -> bool {
		*self == ADDRESS_REGION_PROPERTY_IRI_HTTP || *self == ADDRESS_REGION_PROPERTY_IRI_HTTPS
	}
}
pub struct AddressRegionPropertyIriOrLabel;
impl PartialEq<&str> for AddressRegionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AddressRegionPropertyIri || *other == ADDRESS_REGION_PROPERTY_LABEL
	}
}
impl PartialEq<AddressRegionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AddressRegionPropertyIriOrLabel) -> bool {
		*self == AddressRegionPropertyIri || *self == ADDRESS_REGION_PROPERTY_LABEL
	}
}
