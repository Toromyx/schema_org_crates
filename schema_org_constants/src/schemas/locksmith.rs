/// <https://schema.org/Locksmith>
pub const LOCKSMITH_IRI_HTTP: &str = "http://schema.org/Locksmith";
/// <https://schema.org/Locksmith>
pub const LOCKSMITH_IRI_HTTPS: &str = "https://schema.org/Locksmith";
/// <https://schema.org/Locksmith>
pub const LOCKSMITH_LABEL: &str = "Locksmith";
pub struct LocksmithIri;
impl PartialEq<&str> for LocksmithIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOCKSMITH_IRI_HTTP || *other == LOCKSMITH_IRI_HTTPS
	}
}
impl PartialEq<LocksmithIri> for &str {
	fn eq(&self, other: &LocksmithIri) -> bool {
		*self == LOCKSMITH_IRI_HTTP || *self == LOCKSMITH_IRI_HTTPS
	}
}
pub struct LocksmithIriOrLabel;
impl PartialEq<&str> for LocksmithIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LocksmithIri || *other == LOCKSMITH_LABEL
	}
}
impl PartialEq<LocksmithIriOrLabel> for &str {
	fn eq(&self, other: &LocksmithIriOrLabel) -> bool {
		*self == LocksmithIri || *self == LOCKSMITH_LABEL
	}
}
