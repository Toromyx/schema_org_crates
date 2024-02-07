/// <https://schema.org/LoanOrCredit>
pub trait FindLoanOrCreditIds {
	type IdType;
	/// <https://schema.org/LoanOrCredit>
	fn find_loan_or_credit_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLoanOrCreditIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_loan_or_credit_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LOAN_OR_CREDIT_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LOAN_OR_CREDIT_IRI_HTTPS,
			})
		}
	}
}
