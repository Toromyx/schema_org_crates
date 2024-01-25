/// <https://schema.org/usedToDiagnose>
pub const USED_TO_DIAGNOSE_PROPERTY_IRI_HTTP: &str = "http://schema.org/usedToDiagnose";
/// <https://schema.org/usedToDiagnose>
pub const USED_TO_DIAGNOSE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/usedToDiagnose";
/// <https://schema.org/usedToDiagnose>
pub const USED_TO_DIAGNOSE_PROPERTY_LABEL: &str = "usedToDiagnose";
pub struct UsedToDiagnosePropertyIri;
impl PartialEq<&str> for UsedToDiagnosePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USED_TO_DIAGNOSE_PROPERTY_IRI_HTTP
			|| *other == USED_TO_DIAGNOSE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UsedToDiagnosePropertyIri> for &str {
	fn eq(&self, other: &UsedToDiagnosePropertyIri) -> bool {
		*self == USED_TO_DIAGNOSE_PROPERTY_IRI_HTTP || *self == USED_TO_DIAGNOSE_PROPERTY_IRI_HTTPS
	}
}
pub struct UsedToDiagnosePropertyIriOrLabel;
impl PartialEq<&str> for UsedToDiagnosePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UsedToDiagnosePropertyIri || *other == USED_TO_DIAGNOSE_PROPERTY_LABEL
	}
}
impl PartialEq<UsedToDiagnosePropertyIriOrLabel> for &str {
	fn eq(&self, other: &UsedToDiagnosePropertyIriOrLabel) -> bool {
		*self == UsedToDiagnosePropertyIri || *self == USED_TO_DIAGNOSE_PROPERTY_LABEL
	}
}
