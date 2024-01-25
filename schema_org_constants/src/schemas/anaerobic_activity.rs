/// <https://schema.org/AnaerobicActivity>
pub const ANAEROBIC_ACTIVITY_IRI_HTTP: &str = "http://schema.org/AnaerobicActivity";
/// <https://schema.org/AnaerobicActivity>
pub const ANAEROBIC_ACTIVITY_IRI_HTTPS: &str = "https://schema.org/AnaerobicActivity";
/// <https://schema.org/AnaerobicActivity>
pub const ANAEROBIC_ACTIVITY_LABEL: &str = "AnaerobicActivity";
pub struct AnaerobicActivityIri;
impl PartialEq<&str> for AnaerobicActivityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANAEROBIC_ACTIVITY_IRI_HTTP || *other == ANAEROBIC_ACTIVITY_IRI_HTTPS
	}
}
impl PartialEq<AnaerobicActivityIri> for &str {
	fn eq(&self, other: &AnaerobicActivityIri) -> bool {
		*self == ANAEROBIC_ACTIVITY_IRI_HTTP || *self == ANAEROBIC_ACTIVITY_IRI_HTTPS
	}
}
pub struct AnaerobicActivityIriOrLabel;
impl PartialEq<&str> for AnaerobicActivityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnaerobicActivityIri || *other == ANAEROBIC_ACTIVITY_LABEL
	}
}
impl PartialEq<AnaerobicActivityIriOrLabel> for &str {
	fn eq(&self, other: &AnaerobicActivityIriOrLabel) -> bool {
		*self == AnaerobicActivityIri || *self == ANAEROBIC_ACTIVITY_LABEL
	}
}
