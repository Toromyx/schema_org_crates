/// <https://schema.org/SingleFamilyResidence>
pub trait FindSingleFamilyResidenceIds {
	type IdType;
	/// <https://schema.org/SingleFamilyResidence>
	fn find_single_family_residence_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindSingleFamilyResidenceIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_single_family_residence_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SINGLE_FAMILY_RESIDENCE_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::SINGLE_FAMILY_RESIDENCE_IRI_HTTPS
				}
			})
		}
	}
}
