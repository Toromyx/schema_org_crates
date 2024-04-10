/// <https://schema.org/ShippingDeliveryTime>
pub trait FindShippingDeliveryTimeIds {
	type IdType;
	/// <https://schema.org/ShippingDeliveryTime>
	fn find_shipping_delivery_time_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindShippingDeliveryTimeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_shipping_delivery_time_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SHIPPING_DELIVERY_TIME_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SHIPPING_DELIVERY_TIME_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindShippingDeliveryTimeIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_shipping_delivery_time_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::SHIPPING_DELIVERY_TIME_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::SHIPPING_DELIVERY_TIME_IRI_HTTPS,
			})
		}
	}
}
