/// <https://schema.org/OfferForPurchase>
pub trait FindOfferForPurchaseIds {
	type IdType;
	fn find_offer_for_purchase_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOfferForPurchaseIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_offer_for_purchase_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OFFER_FOR_PURCHASE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OFFER_FOR_PURCHASE_IRI_HTTPS,
			})
		}
	}
}
