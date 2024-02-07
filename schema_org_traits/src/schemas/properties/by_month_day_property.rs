/// <https://schema.org/byMonthDay>
pub trait GetByMonthDayProperty {
	type IdType;
	type PropertyType;
	/// <https://schema.org/byMonthDay>
	fn get_by_month_day_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl crate::GetByMonthDayProperty for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		type PropertyType = rdf_types_0_15::Object;
		fn get_by_month_day_property(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
			self.get_property(
				id,
				match self.namespace() {
					SchemaOrgNamespace::Http => {
						schema_org_constants::BY_MONTH_DAY_PROPERTY_IRI_HTTP
					}
					SchemaOrgNamespace::Https => {
						schema_org_constants::BY_MONTH_DAY_PROPERTY_IRI_HTTPS
					}
				},
			)
		}
	}
}
