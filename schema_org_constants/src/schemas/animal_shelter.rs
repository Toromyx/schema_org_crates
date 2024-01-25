/// <https://schema.org/AnimalShelter>
pub const ANIMAL_SHELTER_IRI_HTTP: &str = "http://schema.org/AnimalShelter";
/// <https://schema.org/AnimalShelter>
pub const ANIMAL_SHELTER_IRI_HTTPS: &str = "https://schema.org/AnimalShelter";
/// <https://schema.org/AnimalShelter>
pub const ANIMAL_SHELTER_LABEL: &str = "AnimalShelter";
pub struct AnimalShelterIri;
impl PartialEq<&str> for AnimalShelterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANIMAL_SHELTER_IRI_HTTP || *other == ANIMAL_SHELTER_IRI_HTTPS
	}
}
impl PartialEq<AnimalShelterIri> for &str {
	fn eq(&self, other: &AnimalShelterIri) -> bool {
		*self == ANIMAL_SHELTER_IRI_HTTP || *self == ANIMAL_SHELTER_IRI_HTTPS
	}
}
pub struct AnimalShelterIriOrLabel;
impl PartialEq<&str> for AnimalShelterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnimalShelterIri || *other == ANIMAL_SHELTER_LABEL
	}
}
impl PartialEq<AnimalShelterIriOrLabel> for &str {
	fn eq(&self, other: &AnimalShelterIriOrLabel) -> bool {
		*self == AnimalShelterIri || *self == ANIMAL_SHELTER_LABEL
	}
}
