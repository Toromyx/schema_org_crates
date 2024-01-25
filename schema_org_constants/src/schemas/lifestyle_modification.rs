/// <https://schema.org/LifestyleModification>
pub const LIFESTYLE_MODIFICATION_IRI_HTTP: &str = "http://schema.org/LifestyleModification";
/// <https://schema.org/LifestyleModification>
pub const LIFESTYLE_MODIFICATION_IRI_HTTPS: &str = "https://schema.org/LifestyleModification";
/// <https://schema.org/LifestyleModification>
pub const LIFESTYLE_MODIFICATION_LABEL: &str = "LifestyleModification";
pub struct LifestyleModificationIri;
impl PartialEq<&str> for LifestyleModificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIFESTYLE_MODIFICATION_IRI_HTTP || *other == LIFESTYLE_MODIFICATION_IRI_HTTPS
	}
}
impl PartialEq<LifestyleModificationIri> for &str {
	fn eq(&self, other: &LifestyleModificationIri) -> bool {
		*self == LIFESTYLE_MODIFICATION_IRI_HTTP || *self == LIFESTYLE_MODIFICATION_IRI_HTTPS
	}
}
pub struct LifestyleModificationIriOrLabel;
impl PartialEq<&str> for LifestyleModificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LifestyleModificationIri || *other == LIFESTYLE_MODIFICATION_LABEL
	}
}
impl PartialEq<LifestyleModificationIriOrLabel> for &str {
	fn eq(&self, other: &LifestyleModificationIriOrLabel) -> bool {
		*self == LifestyleModificationIri || *self == LIFESTYLE_MODIFICATION_LABEL
	}
}
