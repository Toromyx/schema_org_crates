/// <https://schema.org/touristType>
pub const TOURIST_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/touristType";
/// <https://schema.org/touristType>
pub const TOURIST_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/touristType";
/// <https://schema.org/touristType>
pub const TOURIST_TYPE_PROPERTY_LABEL: &str = "touristType";
pub struct TouristTypePropertyIri;
impl PartialEq<&str> for TouristTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOURIST_TYPE_PROPERTY_IRI_HTTP || *other == TOURIST_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TouristTypePropertyIri> for &str {
	fn eq(&self, other: &TouristTypePropertyIri) -> bool {
		*self == TOURIST_TYPE_PROPERTY_IRI_HTTP || *self == TOURIST_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct TouristTypePropertyIriOrLabel;
impl PartialEq<&str> for TouristTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TouristTypePropertyIri || *other == TOURIST_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<TouristTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TouristTypePropertyIriOrLabel) -> bool {
		*self == TouristTypePropertyIri || *self == TOURIST_TYPE_PROPERTY_LABEL
	}
}
