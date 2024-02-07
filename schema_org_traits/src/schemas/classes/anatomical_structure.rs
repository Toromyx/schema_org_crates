/// <https://schema.org/AnatomicalStructure>
pub trait FindAnatomicalStructureIds {
	type IdType;
	/// <https://schema.org/AnatomicalStructure>
	fn find_anatomical_structure_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAnatomicalStructureIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_anatomical_structure_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ANATOMICAL_STRUCTURE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ANATOMICAL_STRUCTURE_IRI_HTTPS,
			})
		}
	}
}
