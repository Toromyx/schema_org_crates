/// <https://schema.org/MusicPlaylist>
pub trait FindMusicPlaylistIds {
	type IdType;
	/// <https://schema.org/MusicPlaylist>
	fn find_music_playlist_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMusicPlaylistIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_music_playlist_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MUSIC_PLAYLIST_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MUSIC_PLAYLIST_IRI_HTTPS,
			})
		}
	}
}
