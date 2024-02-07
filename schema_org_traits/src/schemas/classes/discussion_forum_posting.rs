/// <https://schema.org/DiscussionForumPosting>
pub trait FindDiscussionForumPostingIds {
	type IdType;
	/// <https://schema.org/DiscussionForumPosting>
	fn find_discussion_forum_posting_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDiscussionForumPostingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_discussion_forum_posting_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DISCUSSION_FORUM_POSTING_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::DISCUSSION_FORUM_POSTING_IRI_HTTPS
				}
			})
		}
	}
}
