/// <https://schema.org/PaymentDue>
pub trait FindPaymentDueIds {
	type IdType;
	/// <https://schema.org/PaymentDue>
	fn find_payment_due_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPaymentDueIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_payment_due_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PAYMENT_DUE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PAYMENT_DUE_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPaymentDueIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_payment_due_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PAYMENT_DUE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PAYMENT_DUE_IRI_HTTPS,
			})
		}
	}
}
