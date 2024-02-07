/// <https://schema.org/GenderType>
pub trait FindGenderTypeIds {
	type IdType;
	/// <https://schema.org/GenderType>
	fn find_gender_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindGenderTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_gender_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::GENDER_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::GENDER_TYPE_IRI_HTTPS,
			})
		}
	}
}
