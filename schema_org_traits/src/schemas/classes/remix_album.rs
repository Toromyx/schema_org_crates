/// <https://schema.org/RemixAlbum>
pub trait FindRemixAlbumIds {
	type IdType;
	/// <https://schema.org/RemixAlbum>
	fn find_remix_album_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRemixAlbumIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_remix_album_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REMIX_ALBUM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REMIX_ALBUM_IRI_HTTPS,
			})
		}
	}
}
