/// <https://schema.org/birthPlace>
pub const BIRTH_PLACE_PROPERTY_IRI_HTTP: &str = "http://schema.org/birthPlace";
/// <https://schema.org/birthPlace>
pub const BIRTH_PLACE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/birthPlace";
/// <https://schema.org/birthPlace>
pub const BIRTH_PLACE_PROPERTY_LABEL: &str = "birthPlace";
pub struct BirthPlacePropertyIri;
impl PartialEq<&str> for BirthPlacePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BIRTH_PLACE_PROPERTY_IRI_HTTP || *other == BIRTH_PLACE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BirthPlacePropertyIri> for &str {
	fn eq(&self, other: &BirthPlacePropertyIri) -> bool {
		*self == BIRTH_PLACE_PROPERTY_IRI_HTTP || *self == BIRTH_PLACE_PROPERTY_IRI_HTTPS
	}
}
pub struct BirthPlacePropertyIriOrLabel;
impl PartialEq<&str> for BirthPlacePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BirthPlacePropertyIri || *other == BIRTH_PLACE_PROPERTY_LABEL
	}
}
impl PartialEq<BirthPlacePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BirthPlacePropertyIriOrLabel) -> bool {
		*self == BirthPlacePropertyIri || *self == BIRTH_PLACE_PROPERTY_LABEL
	}
}
