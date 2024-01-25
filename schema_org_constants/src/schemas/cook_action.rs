/// <https://schema.org/CookAction>
pub const COOK_ACTION_IRI_HTTP: &str = "http://schema.org/CookAction";
/// <https://schema.org/CookAction>
pub const COOK_ACTION_IRI_HTTPS: &str = "https://schema.org/CookAction";
/// <https://schema.org/CookAction>
pub const COOK_ACTION_LABEL: &str = "CookAction";
pub struct CookActionIri;
impl PartialEq<&str> for CookActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COOK_ACTION_IRI_HTTP || *other == COOK_ACTION_IRI_HTTPS
	}
}
impl PartialEq<CookActionIri> for &str {
	fn eq(&self, other: &CookActionIri) -> bool {
		*self == COOK_ACTION_IRI_HTTP || *self == COOK_ACTION_IRI_HTTPS
	}
}
pub struct CookActionIriOrLabel;
impl PartialEq<&str> for CookActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CookActionIri || *other == COOK_ACTION_LABEL
	}
}
impl PartialEq<CookActionIriOrLabel> for &str {
	fn eq(&self, other: &CookActionIriOrLabel) -> bool {
		*self == CookActionIri || *self == COOK_ACTION_LABEL
	}
}
