/// <https://schema.org/DepositAccount>
pub trait FindDepositAccountIds {
	type IdType;
	/// <https://schema.org/DepositAccount>
	fn find_deposit_account_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDepositAccountIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_deposit_account_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DEPOSIT_ACCOUNT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DEPOSIT_ACCOUNT_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDepositAccountIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_deposit_account_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DEPOSIT_ACCOUNT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DEPOSIT_ACCOUNT_IRI_HTTPS,
			})
		}
	}
}
