/// <https://schema.org/legislationType>
pub const LEGISLATION_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/legislationType";
/// <https://schema.org/legislationType>
pub const LEGISLATION_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/legislationType";
/// <https://schema.org/legislationType>
pub const LEGISLATION_TYPE_PROPERTY_LABEL: &str = "legislationType";
pub struct LegislationTypePropertyIri;
impl PartialEq<&str> for LegislationTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_TYPE_PROPERTY_IRI_HTTP
			|| *other == LEGISLATION_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LegislationTypePropertyIri> for &str {
	fn eq(&self, other: &LegislationTypePropertyIri) -> bool {
		*self == LEGISLATION_TYPE_PROPERTY_IRI_HTTP || *self == LEGISLATION_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct LegislationTypePropertyIriOrLabel;
impl PartialEq<&str> for LegislationTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationTypePropertyIri || *other == LEGISLATION_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<LegislationTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LegislationTypePropertyIriOrLabel) -> bool {
		*self == LegislationTypePropertyIri || *self == LEGISLATION_TYPE_PROPERTY_LABEL
	}
}
