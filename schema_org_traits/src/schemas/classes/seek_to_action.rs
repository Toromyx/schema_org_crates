/// <https://schema.org/SeekToAction>
pub trait FindSeekToActionIds {
	type IdType;
	/// <https://schema.org/SeekToAction>
	fn find_seek_to_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSeekToActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_seek_to_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SEEK_TO_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SEEK_TO_ACTION_IRI_HTTPS,
			})
		}
	}
}
