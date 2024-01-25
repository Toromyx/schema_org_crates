/// <https://schema.org/DrawAction>
pub const DRAW_ACTION_IRI_HTTP: &str = "http://schema.org/DrawAction";
/// <https://schema.org/DrawAction>
pub const DRAW_ACTION_IRI_HTTPS: &str = "https://schema.org/DrawAction";
/// <https://schema.org/DrawAction>
pub const DRAW_ACTION_LABEL: &str = "DrawAction";
pub struct DrawActionIri;
impl PartialEq<&str> for DrawActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DRAW_ACTION_IRI_HTTP || *other == DRAW_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DrawActionIri> for &str {
	fn eq(&self, other: &DrawActionIri) -> bool {
		*self == DRAW_ACTION_IRI_HTTP || *self == DRAW_ACTION_IRI_HTTPS
	}
}
pub struct DrawActionIriOrLabel;
impl PartialEq<&str> for DrawActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DrawActionIri || *other == DRAW_ACTION_LABEL
	}
}
impl PartialEq<DrawActionIriOrLabel> for &str {
	fn eq(&self, other: &DrawActionIriOrLabel) -> bool {
		*self == DrawActionIri || *self == DRAW_ACTION_LABEL
	}
}
