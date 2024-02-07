/// <https://schema.org/MediaReviewItem>
pub trait FindMediaReviewItemIds {
	type IdType;
	/// <https://schema.org/MediaReviewItem>
	fn find_media_review_item_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMediaReviewItemIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_media_review_item_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDIA_REVIEW_ITEM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDIA_REVIEW_ITEM_IRI_HTTPS,
			})
		}
	}
}
