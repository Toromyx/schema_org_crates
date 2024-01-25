/// <https://schema.org/feesAndCommissionsSpecification>
pub const FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/feesAndCommissionsSpecification";
/// <https://schema.org/feesAndCommissionsSpecification>
pub const FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/feesAndCommissionsSpecification";
/// <https://schema.org/feesAndCommissionsSpecification>
pub const FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_LABEL: &str =
	"feesAndCommissionsSpecification";
pub struct FeesAndCommissionsSpecificationPropertyIri;
impl PartialEq<&str> for FeesAndCommissionsSpecificationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_IRI_HTTP
			|| *other == FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FeesAndCommissionsSpecificationPropertyIri> for &str {
	fn eq(&self, other: &FeesAndCommissionsSpecificationPropertyIri) -> bool {
		*self == FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_IRI_HTTP
			|| *self == FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct FeesAndCommissionsSpecificationPropertyIriOrLabel;
impl PartialEq<&str> for FeesAndCommissionsSpecificationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FeesAndCommissionsSpecificationPropertyIri
			|| *other == FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_LABEL
	}
}
impl PartialEq<FeesAndCommissionsSpecificationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &FeesAndCommissionsSpecificationPropertyIriOrLabel) -> bool {
		*self == FeesAndCommissionsSpecificationPropertyIri
			|| *self == FEES_AND_COMMISSIONS_SPECIFICATION_PROPERTY_LABEL
	}
}
