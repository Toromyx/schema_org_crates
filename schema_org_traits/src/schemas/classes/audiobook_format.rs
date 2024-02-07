/// <https://schema.org/AudiobookFormat>
pub trait FindAudiobookFormatIds {
	type IdType;
	/// <https://schema.org/AudiobookFormat>
	fn find_audiobook_format_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAudiobookFormatIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_audiobook_format_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::AUDIOBOOK_FORMAT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::AUDIOBOOK_FORMAT_IRI_HTTPS,
			})
		}
	}
}
