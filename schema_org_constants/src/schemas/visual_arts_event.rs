/// <https://schema.org/VisualArtsEvent>
pub const VISUAL_ARTS_EVENT_IRI_HTTP: &str = "http://schema.org/VisualArtsEvent";
/// <https://schema.org/VisualArtsEvent>
pub const VISUAL_ARTS_EVENT_IRI_HTTPS: &str = "https://schema.org/VisualArtsEvent";
/// <https://schema.org/VisualArtsEvent>
pub const VISUAL_ARTS_EVENT_LABEL: &str = "VisualArtsEvent";
pub struct VisualArtsEventIri;
impl PartialEq<&str> for VisualArtsEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VISUAL_ARTS_EVENT_IRI_HTTP || *other == VISUAL_ARTS_EVENT_IRI_HTTPS
	}
}
impl PartialEq<VisualArtsEventIri> for &str {
	fn eq(&self, other: &VisualArtsEventIri) -> bool {
		*self == VISUAL_ARTS_EVENT_IRI_HTTP || *self == VISUAL_ARTS_EVENT_IRI_HTTPS
	}
}
pub struct VisualArtsEventIriOrLabel;
impl PartialEq<&str> for VisualArtsEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VisualArtsEventIri || *other == VISUAL_ARTS_EVENT_LABEL
	}
}
impl PartialEq<VisualArtsEventIriOrLabel> for &str {
	fn eq(&self, other: &VisualArtsEventIriOrLabel) -> bool {
		*self == VisualArtsEventIri || *self == VISUAL_ARTS_EVENT_LABEL
	}
}
