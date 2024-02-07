/// <https://schema.org/MolecularEntity>
pub trait FindMolecularEntityIds {
	type IdType;
	/// <https://schema.org/MolecularEntity>
	fn find_molecular_entity_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMolecularEntityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_molecular_entity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MOLECULAR_ENTITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MOLECULAR_ENTITY_IRI_HTTPS,
			})
		}
	}
}
