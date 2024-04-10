/// <https://schema.org/DemoGameAvailability>
pub trait FindDemoGameAvailabilityIds {
	type IdType;
	/// <https://schema.org/DemoGameAvailability>
	fn find_demo_game_availability_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDemoGameAvailabilityIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_demo_game_availability_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DEMO_GAME_AVAILABILITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DEMO_GAME_AVAILABILITY_IRI_HTTPS,
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDemoGameAvailabilityIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_demo_game_availability_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DEMO_GAME_AVAILABILITY_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DEMO_GAME_AVAILABILITY_IRI_HTTPS,
			})
		}
	}
}
