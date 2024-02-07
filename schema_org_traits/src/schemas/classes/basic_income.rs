/// <https://schema.org/BasicIncome>
pub trait FindBasicIncomeIds {
	type IdType;
	/// <https://schema.org/BasicIncome>
	fn find_basic_income_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBasicIncomeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_basic_income_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BASIC_INCOME_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BASIC_INCOME_IRI_HTTPS,
			})
		}
	}
}
