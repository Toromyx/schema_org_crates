/// <https://schema.org/Fungus>
pub trait FindFungusIds {
	type IdType;
	/// <https://schema.org/Fungus>
	fn find_fungus_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFungusIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_fungus_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FUNGUS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FUNGUS_IRI_HTTPS,
			})
		}
	}
}
