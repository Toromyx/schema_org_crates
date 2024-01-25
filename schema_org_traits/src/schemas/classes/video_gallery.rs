/// <https://schema.org/VideoGallery>
pub trait FindVideoGalleryIds {
	type IdType;
	fn find_video_gallery_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVideoGalleryIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_video_gallery_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VIDEO_GALLERY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VIDEO_GALLERY_IRI_HTTPS,
			})
		}
	}
}
