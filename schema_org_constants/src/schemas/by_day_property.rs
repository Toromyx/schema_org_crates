/// <https://schema.org/byDay>
pub const BY_DAY_PROPERTY_IRI_HTTP: &str = "http://schema.org/byDay";
/// <https://schema.org/byDay>
pub const BY_DAY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/byDay";
/// <https://schema.org/byDay>
pub const BY_DAY_PROPERTY_LABEL: &str = "byDay";
pub struct ByDayPropertyIri;
impl PartialEq<&str> for ByDayPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BY_DAY_PROPERTY_IRI_HTTP || *other == BY_DAY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ByDayPropertyIri> for &str {
	fn eq(&self, other: &ByDayPropertyIri) -> bool {
		*self == BY_DAY_PROPERTY_IRI_HTTP || *self == BY_DAY_PROPERTY_IRI_HTTPS
	}
}
pub struct ByDayPropertyIriOrLabel;
impl PartialEq<&str> for ByDayPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ByDayPropertyIri || *other == BY_DAY_PROPERTY_LABEL
	}
}
impl PartialEq<ByDayPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ByDayPropertyIriOrLabel) -> bool {
		*self == ByDayPropertyIri || *self == BY_DAY_PROPERTY_LABEL
	}
}
