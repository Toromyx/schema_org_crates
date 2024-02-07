/// <https://schema.org/SizeSpecification>
pub trait FindSizeSpecificationIds {
	type IdType;
	/// <https://schema.org/SizeSpecification>
	fn find_size_specification_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSizeSpecificationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_size_specification_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SIZE_SPECIFICATION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SIZE_SPECIFICATION_IRI_HTTPS,
			})
		}
	}
}
