/// <https://schema.org/BookmarkAction>
pub trait FindBookmarkActionIds {
	type IdType;
	/// <https://schema.org/BookmarkAction>
	fn find_bookmark_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBookmarkActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_bookmark_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BOOKMARK_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BOOKMARK_ACTION_IRI_HTTPS,
			})
		}
	}
}
