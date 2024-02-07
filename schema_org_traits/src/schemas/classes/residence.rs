/// <https://schema.org/Residence>
pub trait FindResidenceIds {
	type IdType;
	/// <https://schema.org/Residence>
	fn find_residence_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindResidenceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_residence_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RESIDENCE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RESIDENCE_IRI_HTTPS,
			})
		}
	}
}
