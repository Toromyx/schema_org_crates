/// <https://schema.org/LeisureTimeActivity>
pub const LEISURE_TIME_ACTIVITY_IRI_HTTP: &str = "http://schema.org/LeisureTimeActivity";
/// <https://schema.org/LeisureTimeActivity>
pub const LEISURE_TIME_ACTIVITY_IRI_HTTPS: &str = "https://schema.org/LeisureTimeActivity";
/// <https://schema.org/LeisureTimeActivity>
pub const LEISURE_TIME_ACTIVITY_LABEL: &str = "LeisureTimeActivity";
pub struct LeisureTimeActivityIri;
impl PartialEq<&str> for LeisureTimeActivityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEISURE_TIME_ACTIVITY_IRI_HTTP || *other == LEISURE_TIME_ACTIVITY_IRI_HTTPS
	}
}
impl PartialEq<LeisureTimeActivityIri> for &str {
	fn eq(&self, other: &LeisureTimeActivityIri) -> bool {
		*self == LEISURE_TIME_ACTIVITY_IRI_HTTP || *self == LEISURE_TIME_ACTIVITY_IRI_HTTPS
	}
}
pub struct LeisureTimeActivityIriOrLabel;
impl PartialEq<&str> for LeisureTimeActivityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LeisureTimeActivityIri || *other == LEISURE_TIME_ACTIVITY_LABEL
	}
}
impl PartialEq<LeisureTimeActivityIriOrLabel> for &str {
	fn eq(&self, other: &LeisureTimeActivityIriOrLabel) -> bool {
		*self == LeisureTimeActivityIri || *self == LEISURE_TIME_ACTIVITY_LABEL
	}
}
