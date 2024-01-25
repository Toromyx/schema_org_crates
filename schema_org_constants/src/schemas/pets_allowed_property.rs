/// <https://schema.org/petsAllowed>
pub const PETS_ALLOWED_PROPERTY_IRI_HTTP: &str = "http://schema.org/petsAllowed";
/// <https://schema.org/petsAllowed>
pub const PETS_ALLOWED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/petsAllowed";
/// <https://schema.org/petsAllowed>
pub const PETS_ALLOWED_PROPERTY_LABEL: &str = "petsAllowed";
pub struct PetsAllowedPropertyIri;
impl PartialEq<&str> for PetsAllowedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PETS_ALLOWED_PROPERTY_IRI_HTTP || *other == PETS_ALLOWED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PetsAllowedPropertyIri> for &str {
	fn eq(&self, other: &PetsAllowedPropertyIri) -> bool {
		*self == PETS_ALLOWED_PROPERTY_IRI_HTTP || *self == PETS_ALLOWED_PROPERTY_IRI_HTTPS
	}
}
pub struct PetsAllowedPropertyIriOrLabel;
impl PartialEq<&str> for PetsAllowedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PetsAllowedPropertyIri || *other == PETS_ALLOWED_PROPERTY_LABEL
	}
}
impl PartialEq<PetsAllowedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PetsAllowedPropertyIriOrLabel) -> bool {
		*self == PetsAllowedPropertyIri || *self == PETS_ALLOWED_PROPERTY_LABEL
	}
}
