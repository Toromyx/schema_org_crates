/// <https://schema.org/PaymentDeclined>
pub trait FindPaymentDeclinedIds {
	type IdType;
	/// <https://schema.org/PaymentDeclined>
	fn find_payment_declined_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPaymentDeclinedIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_payment_declined_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PAYMENT_DECLINED_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PAYMENT_DECLINED_IRI_HTTPS,
			})
		}
	}
}
