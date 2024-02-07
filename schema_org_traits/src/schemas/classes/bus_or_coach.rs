/// <https://schema.org/BusOrCoach>
pub trait FindBusOrCoachIds {
	type IdType;
	/// <https://schema.org/BusOrCoach>
	fn find_bus_or_coach_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindBusOrCoachIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_bus_or_coach_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::BUS_OR_COACH_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::BUS_OR_COACH_IRI_HTTPS,
			})
		}
	}
}
