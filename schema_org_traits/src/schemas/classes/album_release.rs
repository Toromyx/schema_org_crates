/// <https://schema.org/AlbumRelease>
pub trait FindAlbumReleaseIds {
	type IdType;
	fn find_album_release_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAlbumReleaseIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_album_release_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ALBUM_RELEASE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ALBUM_RELEASE_IRI_HTTPS,
			})
		}
	}
}
