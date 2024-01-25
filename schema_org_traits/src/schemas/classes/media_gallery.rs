/// <https://schema.org/MediaGallery>
pub trait FindMediaGalleryIds {
	type IdType;
	fn find_media_gallery_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMediaGalleryIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_media_gallery_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MEDIA_GALLERY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MEDIA_GALLERY_IRI_HTTPS,
			})
		}
	}
}
