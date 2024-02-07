/// <https://schema.org/VideoGame>
pub trait FindVideoGameIds {
	type IdType;
	/// <https://schema.org/VideoGame>
	fn find_video_game_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVideoGameIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_video_game_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VIDEO_GAME_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VIDEO_GAME_IRI_HTTPS,
			})
		}
	}
}
