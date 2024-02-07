/// <https://schema.org/ReportedDoseSchedule>
pub trait FindReportedDoseScheduleIds {
	type IdType;
	/// <https://schema.org/ReportedDoseSchedule>
	fn find_reported_dose_schedule_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindReportedDoseScheduleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_reported_dose_schedule_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REPORTED_DOSE_SCHEDULE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REPORTED_DOSE_SCHEDULE_IRI_HTTPS,
			})
		}
	}
}
