/// <https://schema.org/Dentistry>
pub trait FindDentistryIds {
	type IdType;
	/// <https://schema.org/Dentistry>
	fn find_dentistry_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDentistryIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_dentistry_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DENTISTRY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DENTISTRY_IRI_HTTPS,
			})
		}
	}
}
