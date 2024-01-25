/// <https://schema.org/MusicAlbumReleaseType>
pub trait FindMusicAlbumReleaseTypeIds {
	type IdType;
	fn find_music_album_release_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMusicAlbumReleaseTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_music_album_release_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MUSIC_ALBUM_RELEASE_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::MUSIC_ALBUM_RELEASE_TYPE_IRI_HTTPS
				}
			})
		}
	}
}
