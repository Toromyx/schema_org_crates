/// <https://schema.org/BankOrCreditUnion>
pub trait FindBankOrCreditUnionIds {
	type IdType;
	/// <https://schema.org/BankOrCreditUnion>
	fn find_bank_or_credit_union_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBankOrCreditUnionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_bank_or_credit_union_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BANK_OR_CREDIT_UNION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BANK_OR_CREDIT_UNION_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBankOrCreditUnionIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_bank_or_credit_union_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BANK_OR_CREDIT_UNION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BANK_OR_CREDIT_UNION_IRI_HTTPS,
			})
		}
	}
}
