/// <https://schema.org/GamePlayMode>
pub trait FindGamePlayModeIds {
	type IdType;
	fn find_game_play_mode_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGamePlayModeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_game_play_mode_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GAME_PLAY_MODE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GAME_PLAY_MODE_IRI_HTTPS,
			})
		}
	}
}
