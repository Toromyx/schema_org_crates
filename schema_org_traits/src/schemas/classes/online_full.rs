/// <https://schema.org/OnlineFull>
pub trait FindOnlineFullIds {
	type IdType;
	/// <https://schema.org/OnlineFull>
	fn find_online_full_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOnlineFullIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_online_full_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ONLINE_FULL_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ONLINE_FULL_IRI_HTTPS,
			})
		}
	}
}
