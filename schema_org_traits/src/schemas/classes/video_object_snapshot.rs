/// <https://schema.org/VideoObjectSnapshot>
pub trait FindVideoObjectSnapshotIds {
	type IdType;
	/// <https://schema.org/VideoObjectSnapshot>
	fn find_video_object_snapshot_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindVideoObjectSnapshotIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_video_object_snapshot_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::VIDEO_OBJECT_SNAPSHOT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::VIDEO_OBJECT_SNAPSHOT_IRI_HTTPS,
			})
		}
	}
}
