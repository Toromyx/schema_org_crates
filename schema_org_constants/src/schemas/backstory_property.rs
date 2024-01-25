/// <https://schema.org/backstory>
pub const BACKSTORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/backstory";
/// <https://schema.org/backstory>
pub const BACKSTORY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/backstory";
/// <https://schema.org/backstory>
pub const BACKSTORY_PROPERTY_LABEL: &str = "backstory";
pub struct BackstoryPropertyIri;
impl PartialEq<&str> for BackstoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BACKSTORY_PROPERTY_IRI_HTTP || *other == BACKSTORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BackstoryPropertyIri> for &str {
	fn eq(&self, other: &BackstoryPropertyIri) -> bool {
		*self == BACKSTORY_PROPERTY_IRI_HTTP || *self == BACKSTORY_PROPERTY_IRI_HTTPS
	}
}
pub struct BackstoryPropertyIriOrLabel;
impl PartialEq<&str> for BackstoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BackstoryPropertyIri || *other == BACKSTORY_PROPERTY_LABEL
	}
}
impl PartialEq<BackstoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BackstoryPropertyIriOrLabel) -> bool {
		*self == BackstoryPropertyIri || *self == BACKSTORY_PROPERTY_LABEL
	}
}
