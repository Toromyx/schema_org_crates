/// <https://schema.org/duration>
pub const DURATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/duration";
/// <https://schema.org/duration>
pub const DURATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/duration";
/// <https://schema.org/duration>
pub const DURATION_PROPERTY_LABEL: &str = "duration";
pub struct DurationPropertyIri;
impl PartialEq<&str> for DurationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DURATION_PROPERTY_IRI_HTTP || *other == DURATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DurationPropertyIri> for &str {
	fn eq(&self, other: &DurationPropertyIri) -> bool {
		*self == DURATION_PROPERTY_IRI_HTTP || *self == DURATION_PROPERTY_IRI_HTTPS
	}
}
pub struct DurationPropertyIriOrLabel;
impl PartialEq<&str> for DurationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DurationPropertyIri || *other == DURATION_PROPERTY_LABEL
	}
}
impl PartialEq<DurationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DurationPropertyIriOrLabel) -> bool {
		*self == DurationPropertyIri || *self == DURATION_PROPERTY_LABEL
	}
}
