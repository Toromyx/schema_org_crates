/// <https://schema.org/DoseSchedule>
pub trait FindDoseScheduleIds {
	type IdType;
	/// <https://schema.org/DoseSchedule>
	fn find_dose_schedule_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDoseScheduleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_dose_schedule_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DOSE_SCHEDULE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DOSE_SCHEDULE_IRI_HTTPS,
			})
		}
	}
}
