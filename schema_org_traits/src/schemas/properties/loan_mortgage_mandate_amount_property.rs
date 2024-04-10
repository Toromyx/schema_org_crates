/// <https://schema.org/loanMortgageMandateAmount>
pub trait GetLoanMortgageMandateAmountProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/loanMortgageMandateAmount>
	fn get_loan_mortgage_mandate_amount_property(
		&self,
		id: &Self::IdType,
	) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetLoanMortgageMandateAmountProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_loan_mortgage_mandate_amount_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetLoanMortgageMandateAmountProperty for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		type PropertyType = rdf_types_0_22::Object;
		fn get_loan_mortgage_mandate_amount_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::LOAN_MORTGAGE_MANDATE_AMOUNT_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
