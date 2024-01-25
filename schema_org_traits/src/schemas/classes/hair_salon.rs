/// <https://schema.org/HairSalon>
pub trait FindHairSalonIds {
	type IdType;
	fn find_hair_salon_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHairSalonIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_hair_salon_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::HAIR_SALON_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::HAIR_SALON_IRI_HTTPS,
			})
		}
	}
}
