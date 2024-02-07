/// <https://schema.org/Integer>
pub trait FindIntegerIds {
	type IdType;
	/// <https://schema.org/Integer>
	fn find_integer_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindIntegerIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_integer_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INTEGER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INTEGER_IRI_HTTPS,
			})
		}
	}
}
