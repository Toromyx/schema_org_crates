/// <https://schema.org/ReturnShippingFees>
pub trait FindReturnShippingFeesIds {
	type IdType;
	/// <https://schema.org/ReturnShippingFees>
	fn find_return_shipping_fees_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReturnShippingFeesIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_return_shipping_fees_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RETURN_SHIPPING_FEES_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RETURN_SHIPPING_FEES_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReturnShippingFeesIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_return_shipping_fees_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::RETURN_SHIPPING_FEES_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::RETURN_SHIPPING_FEES_IRI_HTTPS,
			})
		}
	}
}
