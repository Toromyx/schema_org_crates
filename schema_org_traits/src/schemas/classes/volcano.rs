/// <https://schema.org/Volcano>
pub trait FindVolcanoIds {
	type IdType;
	/// <https://schema.org/Volcano>
	fn find_volcano_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVolcanoIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_volcano_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VOLCANO_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VOLCANO_IRI_HTTPS,
			})
		}
	}
}
