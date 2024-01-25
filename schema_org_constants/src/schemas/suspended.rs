/// <https://schema.org/Suspended>
pub const SUSPENDED_IRI_HTTP: &str = "http://schema.org/Suspended";
/// <https://schema.org/Suspended>
pub const SUSPENDED_IRI_HTTPS: &str = "https://schema.org/Suspended";
/// <https://schema.org/Suspended>
pub const SUSPENDED_LABEL: &str = "Suspended";
pub struct SuspendedIri;
impl PartialEq<&str> for SuspendedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUSPENDED_IRI_HTTP || *other == SUSPENDED_IRI_HTTPS
	}
}
impl PartialEq<SuspendedIri> for &str {
	fn eq(&self, other: &SuspendedIri) -> bool {
		*self == SUSPENDED_IRI_HTTP || *self == SUSPENDED_IRI_HTTPS
	}
}
pub struct SuspendedIriOrLabel;
impl PartialEq<&str> for SuspendedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuspendedIri || *other == SUSPENDED_LABEL
	}
}
impl PartialEq<SuspendedIriOrLabel> for &str {
	fn eq(&self, other: &SuspendedIriOrLabel) -> bool {
		*self == SuspendedIri || *self == SUSPENDED_LABEL
	}
}
