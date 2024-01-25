/// <https://schema.org/UseAction>
pub const USE_ACTION_IRI_HTTP: &str = "http://schema.org/UseAction";
/// <https://schema.org/UseAction>
pub const USE_ACTION_IRI_HTTPS: &str = "https://schema.org/UseAction";
/// <https://schema.org/UseAction>
pub const USE_ACTION_LABEL: &str = "UseAction";
pub struct UseActionIri;
impl PartialEq<&str> for UseActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USE_ACTION_IRI_HTTP || *other == USE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<UseActionIri> for &str {
	fn eq(&self, other: &UseActionIri) -> bool {
		*self == USE_ACTION_IRI_HTTP || *self == USE_ACTION_IRI_HTTPS
	}
}
pub struct UseActionIriOrLabel;
impl PartialEq<&str> for UseActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UseActionIri || *other == USE_ACTION_LABEL
	}
}
impl PartialEq<UseActionIriOrLabel> for &str {
	fn eq(&self, other: &UseActionIriOrLabel) -> bool {
		*self == UseActionIri || *self == USE_ACTION_LABEL
	}
}
