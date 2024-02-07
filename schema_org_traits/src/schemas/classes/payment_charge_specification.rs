/// <https://schema.org/PaymentChargeSpecification>
pub trait FindPaymentChargeSpecificationIds {
	type IdType;
	/// <https://schema.org/PaymentChargeSpecification>
	fn find_payment_charge_specification_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPaymentChargeSpecificationIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_payment_charge_specification_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::PAYMENT_CHARGE_SPECIFICATION_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::PAYMENT_CHARGE_SPECIFICATION_IRI_HTTPS
				}
			})
		}
	}
}
