/// <https://schema.org/NotInForce>
pub const NOT_IN_FORCE_IRI_HTTP: &str = "http://schema.org/NotInForce";
/// <https://schema.org/NotInForce>
pub const NOT_IN_FORCE_IRI_HTTPS: &str = "https://schema.org/NotInForce";
/// <https://schema.org/NotInForce>
pub const NOT_IN_FORCE_LABEL: &str = "NotInForce";
pub struct NotInForceIri;
impl PartialEq<&str> for NotInForceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NOT_IN_FORCE_IRI_HTTP || *other == NOT_IN_FORCE_IRI_HTTPS
	}
}
impl PartialEq<NotInForceIri> for &str {
	fn eq(&self, other: &NotInForceIri) -> bool {
		*self == NOT_IN_FORCE_IRI_HTTP || *self == NOT_IN_FORCE_IRI_HTTPS
	}
}
pub struct NotInForceIriOrLabel;
impl PartialEq<&str> for NotInForceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NotInForceIri || *other == NOT_IN_FORCE_LABEL
	}
}
impl PartialEq<NotInForceIriOrLabel> for &str {
	fn eq(&self, other: &NotInForceIriOrLabel) -> bool {
		*self == NotInForceIri || *self == NOT_IN_FORCE_LABEL
	}
}
