/// <https://schema.org/Class>
pub trait FindClassIds {
	type IdType;
	fn find_class_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindClassIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_class_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CLASS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CLASS_IRI_HTTPS,
			})
		}
	}
}
