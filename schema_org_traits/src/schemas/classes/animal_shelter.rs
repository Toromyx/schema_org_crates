/// <https://schema.org/AnimalShelter>
pub trait FindAnimalShelterIds {
	type IdType;
	fn find_animal_shelter_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAnimalShelterIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_animal_shelter_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ANIMAL_SHELTER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ANIMAL_SHELTER_IRI_HTTPS,
			})
		}
	}
}
