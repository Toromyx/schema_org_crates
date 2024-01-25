/// <https://schema.org/BusOrCoach>
pub const BUS_OR_COACH_IRI_HTTP: &str = "http://schema.org/BusOrCoach";
/// <https://schema.org/BusOrCoach>
pub const BUS_OR_COACH_IRI_HTTPS: &str = "https://schema.org/BusOrCoach";
/// <https://schema.org/BusOrCoach>
pub const BUS_OR_COACH_LABEL: &str = "BusOrCoach";
pub struct BusOrCoachIri;
impl PartialEq<&str> for BusOrCoachIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUS_OR_COACH_IRI_HTTP || *other == BUS_OR_COACH_IRI_HTTPS
	}
}
impl PartialEq<BusOrCoachIri> for &str {
	fn eq(&self, other: &BusOrCoachIri) -> bool {
		*self == BUS_OR_COACH_IRI_HTTP || *self == BUS_OR_COACH_IRI_HTTPS
	}
}
pub struct BusOrCoachIriOrLabel;
impl PartialEq<&str> for BusOrCoachIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusOrCoachIri || *other == BUS_OR_COACH_LABEL
	}
}
impl PartialEq<BusOrCoachIriOrLabel> for &str {
	fn eq(&self, other: &BusOrCoachIriOrLabel) -> bool {
		*self == BusOrCoachIri || *self == BUS_OR_COACH_LABEL
	}
}
