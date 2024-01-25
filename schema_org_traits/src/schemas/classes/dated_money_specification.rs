/// <https://schema.org/DatedMoneySpecification>
#[deprecated = "This schema is superseded by <https://schema.org/MonetaryAmount>."]
pub trait FindDatedMoneySpecificationIds {
	type IdType;
	#[deprecated = "This schema is superseded by <https://schema.org/MonetaryAmount>."]
	fn find_dated_money_specification_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDatedMoneySpecificationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_dated_money_specification_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::DATED_MONEY_SPECIFICATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::DATED_MONEY_SPECIFICATION_IRI_HTTPS
				}
			})
		}
	}
}
