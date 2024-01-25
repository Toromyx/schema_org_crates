/// <https://schema.org/WesternConventional>
pub trait FindWesternConventionalIds {
	type IdType;
	fn find_western_conventional_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWesternConventionalIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_western_conventional_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WESTERN_CONVENTIONAL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WESTERN_CONVENTIONAL_IRI_HTTPS,
			})
		}
	}
}
