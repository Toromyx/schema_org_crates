/// <https://schema.org/OneTimePayments>
pub trait FindOneTimePaymentsIds {
	type IdType;
	/// <https://schema.org/OneTimePayments>
	fn find_one_time_payments_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOneTimePaymentsIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_one_time_payments_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ONE_TIME_PAYMENTS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ONE_TIME_PAYMENTS_IRI_HTTPS,
			})
		}
	}
}
