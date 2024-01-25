/// <https://schema.org/statType>
pub const STAT_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/statType";
/// <https://schema.org/statType>
pub const STAT_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/statType";
/// <https://schema.org/statType>
pub const STAT_TYPE_PROPERTY_LABEL: &str = "statType";
pub struct StatTypePropertyIri;
impl PartialEq<&str> for StatTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STAT_TYPE_PROPERTY_IRI_HTTP || *other == STAT_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StatTypePropertyIri> for &str {
	fn eq(&self, other: &StatTypePropertyIri) -> bool {
		*self == STAT_TYPE_PROPERTY_IRI_HTTP || *self == STAT_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct StatTypePropertyIriOrLabel;
impl PartialEq<&str> for StatTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StatTypePropertyIri || *other == STAT_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<StatTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &StatTypePropertyIriOrLabel) -> bool {
		*self == StatTypePropertyIri || *self == STAT_TYPE_PROPERTY_LABEL
	}
}
