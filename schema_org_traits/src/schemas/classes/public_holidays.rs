/// <https://schema.org/PublicHolidays>
pub trait FindPublicHolidaysIds {
	type IdType;
	/// <https://schema.org/PublicHolidays>
	fn find_public_holidays_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindPublicHolidaysIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_public_holidays_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::PUBLIC_HOLIDAYS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::PUBLIC_HOLIDAYS_IRI_HTTPS,
			})
		}
	}
}
