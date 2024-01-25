/// <https://schema.org/isInvolvedInBiologicalProcess>
pub const IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/isInvolvedInBiologicalProcess";
/// <https://schema.org/isInvolvedInBiologicalProcess>
pub const IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isInvolvedInBiologicalProcess";
/// <https://schema.org/isInvolvedInBiologicalProcess>
pub const IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_LABEL: &str = "isInvolvedInBiologicalProcess";
pub struct IsInvolvedInBiologicalProcessPropertyIri;
impl PartialEq<&str> for IsInvolvedInBiologicalProcessPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_IRI_HTTP
			|| *other == IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsInvolvedInBiologicalProcessPropertyIri> for &str {
	fn eq(&self, other: &IsInvolvedInBiologicalProcessPropertyIri) -> bool {
		*self == IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_IRI_HTTP
			|| *self == IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_IRI_HTTPS
	}
}
pub struct IsInvolvedInBiologicalProcessPropertyIriOrLabel;
impl PartialEq<&str> for IsInvolvedInBiologicalProcessPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsInvolvedInBiologicalProcessPropertyIri
			|| *other == IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_LABEL
	}
}
impl PartialEq<IsInvolvedInBiologicalProcessPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsInvolvedInBiologicalProcessPropertyIriOrLabel) -> bool {
		*self == IsInvolvedInBiologicalProcessPropertyIri
			|| *self == IS_INVOLVED_IN_BIOLOGICAL_PROCESS_PROPERTY_LABEL
	}
}
