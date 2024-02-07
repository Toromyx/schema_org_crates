/// <https://schema.org/numberOfAvailableAccommodationUnits>
pub trait GetNumberOfAvailableAccommodationUnitsProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/numberOfAvailableAccommodationUnits>
	fn get_number_of_available_accommodation_units_property(
		&self,
		id: &Self::IdType,
	) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetNumberOfAvailableAccommodationUnitsProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_number_of_available_accommodation_units_property(
			&self,
			id: &Self::IdType,
		) -> Vec<&Self::PropertyType> {
			self.get_property(
                id,
                match self.namespace() {
                    SchemaOrgNamespace::Http => {
                        schema_org_constants::NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTP
                    }
                    SchemaOrgNamespace::Https => {
                        schema_org_constants::NUMBER_OF_AVAILABLE_ACCOMMODATION_UNITS_PROPERTY_IRI_HTTPS
                    }
                },
            )
		}
	}
}
