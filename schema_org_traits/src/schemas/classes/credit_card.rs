/// <https://schema.org/CreditCard>
pub trait FindCreditCardIds {
	type IdType;
	/// <https://schema.org/CreditCard>
	fn find_credit_card_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCreditCardIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_credit_card_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CREDIT_CARD_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CREDIT_CARD_IRI_HTTPS,
			})
		}
	}
}
