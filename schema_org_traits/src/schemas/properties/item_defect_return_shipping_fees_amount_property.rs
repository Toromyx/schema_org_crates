/// <https://schema.org/itemDefectReturnShippingFeesAmount>
pub trait GetItemDefectReturnShippingFeesAmountProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/itemDefectReturnShippingFeesAmount>
	fn get_item_defect_return_shipping_fees_amount_property(
		&self,
		id: &Self::IdType,
	) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetItemDefectReturnShippingFeesAmountProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_item_defect_return_shipping_fees_amount_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
                id,
                match self.namespace() {
                    SchemaOrgNamespace::Http => {
                        schema_org_constants::ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTP
                    }
                    SchemaOrgNamespace::Https => {
                        schema_org_constants::ITEM_DEFECT_RETURN_SHIPPING_FEES_AMOUNT_PROPERTY_IRI_HTTPS
                    }
                },
            )
		}
	}
}
