/// <https://schema.org/maps>
#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
pub const MAPS_PROPERTY_IRI_HTTP: &str = "http://schema.org/maps";
/// <https://schema.org/maps>
#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
pub const MAPS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/maps";
/// <https://schema.org/maps>
#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
pub const MAPS_PROPERTY_LABEL: &str = "maps";
pub struct MapsPropertyIri;
impl PartialEq<&str> for MapsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAPS_PROPERTY_IRI_HTTP || *other == MAPS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MapsPropertyIri> for &str {
	fn eq(&self, other: &MapsPropertyIri) -> bool {
		*self == MAPS_PROPERTY_IRI_HTTP || *self == MAPS_PROPERTY_IRI_HTTPS
	}
}
pub struct MapsPropertyIriOrLabel;
impl PartialEq<&str> for MapsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MapsPropertyIri || *other == MAPS_PROPERTY_LABEL
	}
}
impl PartialEq<MapsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MapsPropertyIriOrLabel) -> bool {
		*self == MapsPropertyIri || *self == MAPS_PROPERTY_LABEL
	}
}
