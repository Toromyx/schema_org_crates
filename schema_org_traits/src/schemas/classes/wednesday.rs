/// <https://schema.org/Wednesday>
pub trait FindWednesdayIds {
	type IdType;
	/// <https://schema.org/Wednesday>
	fn find_wednesday_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWednesdayIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_wednesday_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WEDNESDAY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WEDNESDAY_IRI_HTTPS,
			})
		}
	}
}
