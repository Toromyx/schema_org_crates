/// <https://schema.org/Cardiovascular>
pub trait FindCardiovascularIds {
	type IdType;
	/// <https://schema.org/Cardiovascular>
	fn find_cardiovascular_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCardiovascularIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_cardiovascular_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CARDIOVASCULAR_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CARDIOVASCULAR_IRI_HTTPS,
			})
		}
	}
}
