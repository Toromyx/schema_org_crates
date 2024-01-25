/// <https://schema.org/latitude>
pub const LATITUDE_PROPERTY_IRI_HTTP: &str = "http://schema.org/latitude";
/// <https://schema.org/latitude>
pub const LATITUDE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/latitude";
/// <https://schema.org/latitude>
pub const LATITUDE_PROPERTY_LABEL: &str = "latitude";
pub struct LatitudePropertyIri;
impl PartialEq<&str> for LatitudePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LATITUDE_PROPERTY_IRI_HTTP || *other == LATITUDE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LatitudePropertyIri> for &str {
	fn eq(&self, other: &LatitudePropertyIri) -> bool {
		*self == LATITUDE_PROPERTY_IRI_HTTP || *self == LATITUDE_PROPERTY_IRI_HTTPS
	}
}
pub struct LatitudePropertyIriOrLabel;
impl PartialEq<&str> for LatitudePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LatitudePropertyIri || *other == LATITUDE_PROPERTY_LABEL
	}
}
impl PartialEq<LatitudePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LatitudePropertyIriOrLabel) -> bool {
		*self == LatitudePropertyIri || *self == LATITUDE_PROPERTY_LABEL
	}
}
