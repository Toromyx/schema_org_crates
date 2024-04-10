/// <https://schema.org/PetStore>
pub trait FindPetStoreIds {
	type IdType;
	/// <https://schema.org/PetStore>
	fn find_pet_store_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPetStoreIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_pet_store_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PET_STORE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PET_STORE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPetStoreIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_pet_store_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PET_STORE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PET_STORE_IRI_HTTPS,
			})
		}
	}
}
