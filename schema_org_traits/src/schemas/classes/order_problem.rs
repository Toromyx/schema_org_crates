/// <https://schema.org/OrderProblem>
pub trait FindOrderProblemIds {
	type IdType;
	/// <https://schema.org/OrderProblem>
	fn find_order_problem_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOrderProblemIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_order_problem_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ORDER_PROBLEM_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ORDER_PROBLEM_IRI_HTTPS,
			})
		}
	}
}
