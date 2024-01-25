/// <https://schema.org/Renal>
pub const RENAL_IRI_HTTP: &str = "http://schema.org/Renal";
/// <https://schema.org/Renal>
pub const RENAL_IRI_HTTPS: &str = "https://schema.org/Renal";
/// <https://schema.org/Renal>
pub const RENAL_LABEL: &str = "Renal";
pub struct RenalIri;
impl PartialEq<&str> for RenalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RENAL_IRI_HTTP || *other == RENAL_IRI_HTTPS
	}
}
impl PartialEq<RenalIri> for &str {
	fn eq(&self, other: &RenalIri) -> bool {
		*self == RENAL_IRI_HTTP || *self == RENAL_IRI_HTTPS
	}
}
pub struct RenalIriOrLabel;
impl PartialEq<&str> for RenalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RenalIri || *other == RENAL_LABEL
	}
}
impl PartialEq<RenalIriOrLabel> for &str {
	fn eq(&self, other: &RenalIriOrLabel) -> bool {
		*self == RenalIri || *self == RENAL_LABEL
	}
}
