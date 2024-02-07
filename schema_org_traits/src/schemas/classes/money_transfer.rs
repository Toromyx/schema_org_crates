/// <https://schema.org/MoneyTransfer>
pub trait FindMoneyTransferIds {
	type IdType;
	/// <https://schema.org/MoneyTransfer>
	fn find_money_transfer_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMoneyTransferIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_money_transfer_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MONEY_TRANSFER_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MONEY_TRANSFER_IRI_HTTPS,
			})
		}
	}
}
