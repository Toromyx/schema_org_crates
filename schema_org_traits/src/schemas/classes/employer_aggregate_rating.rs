/// <https://schema.org/EmployerAggregateRating>
pub trait FindEmployerAggregateRatingIds {
	type IdType;
	/// <https://schema.org/EmployerAggregateRating>
	fn find_employer_aggregate_rating_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEmployerAggregateRatingIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_employer_aggregate_rating_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::EMPLOYER_AGGREGATE_RATING_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::EMPLOYER_AGGREGATE_RATING_IRI_HTTPS
				}
			})
		}
	}
}
#[cfg(any(feature = "json-ld_0_16", doc))]
mod json_ld_0_16 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEmployerAggregateRatingIds for crate::json_ld_0_16::JsonLdStore {
		type IdType = json_ld_0_16::ValidId;
		fn find_employer_aggregate_rating_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => {
					schema_org_constants::EMPLOYER_AGGREGATE_RATING_IRI_HTTP
				}
				SchemaOrgNamespace::Https => {
					schema_org_constants::EMPLOYER_AGGREGATE_RATING_IRI_HTTPS
				}
			})
		}
	}
}
