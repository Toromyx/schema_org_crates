/// <https://schema.org/Withdrawn>
pub const WITHDRAWN_IRI_HTTP: &str = "http://schema.org/Withdrawn";
/// <https://schema.org/Withdrawn>
pub const WITHDRAWN_IRI_HTTPS: &str = "https://schema.org/Withdrawn";
/// <https://schema.org/Withdrawn>
pub const WITHDRAWN_LABEL: &str = "Withdrawn";
pub struct WithdrawnIri;
impl PartialEq<&str> for WithdrawnIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WITHDRAWN_IRI_HTTP || *other == WITHDRAWN_IRI_HTTPS
	}
}
impl PartialEq<WithdrawnIri> for &str {
	fn eq(&self, other: &WithdrawnIri) -> bool {
		*self == WITHDRAWN_IRI_HTTP || *self == WITHDRAWN_IRI_HTTPS
	}
}
pub struct WithdrawnIriOrLabel;
impl PartialEq<&str> for WithdrawnIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WithdrawnIri || *other == WITHDRAWN_LABEL
	}
}
impl PartialEq<WithdrawnIriOrLabel> for &str {
	fn eq(&self, other: &WithdrawnIriOrLabel) -> bool {
		*self == WithdrawnIri || *self == WITHDRAWN_LABEL
	}
}
