/// <https://schema.org/OfferItemCondition>
pub trait FindOfferItemConditionIds {
	type IdType;
	/// <https://schema.org/OfferItemCondition>
	fn find_offer_item_condition_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOfferItemConditionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_offer_item_condition_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OFFER_ITEM_CONDITION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OFFER_ITEM_CONDITION_IRI_HTTPS,
			})
		}
	}
}
