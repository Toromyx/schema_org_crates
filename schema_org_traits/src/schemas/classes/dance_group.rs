/// <https://schema.org/DanceGroup>
pub trait FindDanceGroupIds {
	type IdType;
	/// <https://schema.org/DanceGroup>
	fn find_dance_group_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDanceGroupIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_dance_group_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DANCE_GROUP_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DANCE_GROUP_IRI_HTTPS,
			})
		}
	}
}
