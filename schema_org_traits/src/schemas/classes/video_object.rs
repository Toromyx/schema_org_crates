/// <https://schema.org/VideoObject>
pub trait FindVideoObjectIds {
	type IdType;
	fn find_video_object_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVideoObjectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_video_object_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VIDEO_OBJECT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VIDEO_OBJECT_IRI_HTTPS,
			})
		}
	}
}
