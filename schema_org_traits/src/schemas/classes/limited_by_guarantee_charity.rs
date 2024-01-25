/// <https://schema.org/LimitedByGuaranteeCharity>
pub trait FindLimitedByGuaranteeCharityIds {
	type IdType;
	fn find_limited_by_guarantee_charity_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLimitedByGuaranteeCharityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_limited_by_guarantee_charity_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::LIMITED_BY_GUARANTEE_CHARITY_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::LIMITED_BY_GUARANTEE_CHARITY_IRI_HTTPS
				}
			})
		}
	}
}
