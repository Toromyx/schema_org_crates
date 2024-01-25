/// <https://schema.org/Neurologic>
pub trait FindNeurologicIds {
	type IdType;
	fn find_neurologic_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNeurologicIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_neurologic_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NEUROLOGIC_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NEUROLOGIC_IRI_HTTPS,
			})
		}
	}
}
