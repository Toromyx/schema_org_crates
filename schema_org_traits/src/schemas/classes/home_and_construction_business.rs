/// <https://schema.org/HomeAndConstructionBusiness>
pub trait FindHomeAndConstructionBusinessIds {
	type IdType;
	/// <https://schema.org/HomeAndConstructionBusiness>
	fn find_home_and_construction_business_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindHomeAndConstructionBusinessIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_home_and_construction_business_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::HOME_AND_CONSTRUCTION_BUSINESS_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::HOME_AND_CONSTRUCTION_BUSINESS_IRI_HTTPS
				}
			})
		}
	}
}
