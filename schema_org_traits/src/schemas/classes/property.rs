/// <https://schema.org/Property>
pub trait FindPropertyIds {
	type IdType;
	fn find_property_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPropertyIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_property_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PROPERTY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PROPERTY_IRI_HTTPS,
			})
		}
	}
}
