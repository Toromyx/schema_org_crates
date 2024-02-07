/// <https://schema.org/itinerary>
pub trait GetItineraryProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/itinerary>
	fn get_itinerary_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetItineraryProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_itinerary_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => schema_org_constants::ITINERARY_PROPERTY_IRI_HTTP,
					SchemaOrgNamespace::Https => schema_org_constants::ITINERARY_PROPERTY_IRI_HTTPS,
				},
			)
		}
	}
}
