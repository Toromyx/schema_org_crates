/// <https://schema.org/EventMovedOnline>
pub trait FindEventMovedOnlineIds {
	type IdType;
	/// <https://schema.org/EventMovedOnline>
	fn find_event_moved_online_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEventMovedOnlineIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_event_moved_online_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EVENT_MOVED_ONLINE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EVENT_MOVED_ONLINE_IRI_HTTPS,
			})
		}
	}
}
