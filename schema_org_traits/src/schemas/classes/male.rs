/// <https://schema.org/Male>
pub trait FindMaleIds {
	type IdType;
	fn find_male_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMaleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_male_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MALE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MALE_IRI_HTTPS,
			})
		}
	}
}
