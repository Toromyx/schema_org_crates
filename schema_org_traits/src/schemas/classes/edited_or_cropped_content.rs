/// <https://schema.org/EditedOrCroppedContent>
pub trait FindEditedOrCroppedContentIds {
	type IdType;
	/// <https://schema.org/EditedOrCroppedContent>
	fn find_edited_or_cropped_content_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEditedOrCroppedContentIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_edited_or_cropped_content_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::EDITED_OR_CROPPED_CONTENT_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::EDITED_OR_CROPPED_CONTENT_IRI_HTTPS
				}
			})
		}
	}
}
