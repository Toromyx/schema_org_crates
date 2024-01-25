/// <https://schema.org/CivicStructure>
pub trait FindCivicStructureIds {
	type IdType;
	fn find_civic_structure_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCivicStructureIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_civic_structure_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CIVIC_STRUCTURE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CIVIC_STRUCTURE_IRI_HTTPS,
			})
		}
	}
}
