/// <https://schema.org/longitude>
pub const LONGITUDE_PROPERTY_IRI_HTTP: &str = "http://schema.org/longitude";
/// <https://schema.org/longitude>
pub const LONGITUDE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/longitude";
/// <https://schema.org/longitude>
pub const LONGITUDE_PROPERTY_LABEL: &str = "longitude";
pub struct LongitudePropertyIri;
impl PartialEq<&str> for LongitudePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LONGITUDE_PROPERTY_IRI_HTTP || *other == LONGITUDE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LongitudePropertyIri> for &str {
	fn eq(&self, other: &LongitudePropertyIri) -> bool {
		*self == LONGITUDE_PROPERTY_IRI_HTTP || *self == LONGITUDE_PROPERTY_IRI_HTTPS
	}
}
pub struct LongitudePropertyIriOrLabel;
impl PartialEq<&str> for LongitudePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LongitudePropertyIri || *other == LONGITUDE_PROPERTY_LABEL
	}
}
impl PartialEq<LongitudePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LongitudePropertyIriOrLabel) -> bool {
		*self == LongitudePropertyIri || *self == LONGITUDE_PROPERTY_LABEL
	}
}
