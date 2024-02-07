/// <https://schema.org/UsageOrScheduleHealthAspect>
pub trait FindUsageOrScheduleHealthAspectIds {
	type IdType;
	/// <https://schema.org/UsageOrScheduleHealthAspect>
	fn find_usage_or_schedule_health_aspect_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUsageOrScheduleHealthAspectIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_usage_or_schedule_health_aspect_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::USAGE_OR_SCHEDULE_HEALTH_ASPECT_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::USAGE_OR_SCHEDULE_HEALTH_ASPECT_IRI_HTTPS
				}
			})
		}
	}
}
