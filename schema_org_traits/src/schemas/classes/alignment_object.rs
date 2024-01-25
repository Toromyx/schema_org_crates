/// <https://schema.org/AlignmentObject>
pub trait FindAlignmentObjectIds {
	type IdType;
	fn find_alignment_object_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAlignmentObjectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_alignment_object_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ALIGNMENT_OBJECT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ALIGNMENT_OBJECT_IRI_HTTPS,
			})
		}
	}
}
