/// <https://schema.org/dayOfWeek>
pub trait GetDayOfWeekProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/dayOfWeek>
	fn get_day_of_week_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetDayOfWeekProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_day_of_week_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => schema_org_constants::DAY_OF_WEEK_PROPERTY_IRI_HTTP,
					SchemaOrgNamespace::Https => {
						schema_org_constants::DAY_OF_WEEK_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
