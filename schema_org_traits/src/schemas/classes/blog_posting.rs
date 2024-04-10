/// <https://schema.org/BlogPosting>
pub trait FindBlogPostingIds {
	type IdType;
	/// <https://schema.org/BlogPosting>
	fn find_blog_posting_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBlogPostingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_blog_posting_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BLOG_POSTING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BLOG_POSTING_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBlogPostingIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_blog_posting_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BLOG_POSTING_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BLOG_POSTING_IRI_HTTPS,
			})
		}
	}
}
