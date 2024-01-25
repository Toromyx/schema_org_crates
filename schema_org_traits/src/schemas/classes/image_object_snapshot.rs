/// <https://schema.org/ImageObjectSnapshot>
pub trait FindImageObjectSnapshotIds {
	type IdType;
	fn find_image_object_snapshot_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindImageObjectSnapshotIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_image_object_snapshot_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::IMAGE_OBJECT_SNAPSHOT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::IMAGE_OBJECT_SNAPSHOT_IRI_HTTPS,
			})
		}
	}
}
