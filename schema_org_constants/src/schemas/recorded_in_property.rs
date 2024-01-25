/// <https://schema.org/recordedIn>
pub const RECORDED_IN_PROPERTY_IRI_HTTP: &str = "http://schema.org/recordedIn";
/// <https://schema.org/recordedIn>
pub const RECORDED_IN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recordedIn";
/// <https://schema.org/recordedIn>
pub const RECORDED_IN_PROPERTY_LABEL: &str = "recordedIn";
pub struct RecordedInPropertyIri;
impl PartialEq<&str> for RecordedInPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECORDED_IN_PROPERTY_IRI_HTTP || *other == RECORDED_IN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecordedInPropertyIri> for &str {
	fn eq(&self, other: &RecordedInPropertyIri) -> bool {
		*self == RECORDED_IN_PROPERTY_IRI_HTTP || *self == RECORDED_IN_PROPERTY_IRI_HTTPS
	}
}
pub struct RecordedInPropertyIriOrLabel;
impl PartialEq<&str> for RecordedInPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecordedInPropertyIri || *other == RECORDED_IN_PROPERTY_LABEL
	}
}
impl PartialEq<RecordedInPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecordedInPropertyIriOrLabel) -> bool {
		*self == RecordedInPropertyIri || *self == RECORDED_IN_PROPERTY_LABEL
	}
}
