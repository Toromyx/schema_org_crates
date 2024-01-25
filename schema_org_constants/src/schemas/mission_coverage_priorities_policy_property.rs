/// <https://schema.org/missionCoveragePrioritiesPolicy>
pub const MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/missionCoveragePrioritiesPolicy";
/// <https://schema.org/missionCoveragePrioritiesPolicy>
pub const MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/missionCoveragePrioritiesPolicy";
/// <https://schema.org/missionCoveragePrioritiesPolicy>
pub const MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_LABEL: &str =
	"missionCoveragePrioritiesPolicy";
pub struct MissionCoveragePrioritiesPolicyPropertyIri;
impl PartialEq<&str> for MissionCoveragePrioritiesPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_IRI_HTTP
			|| *other == MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MissionCoveragePrioritiesPolicyPropertyIri> for &str {
	fn eq(&self, other: &MissionCoveragePrioritiesPolicyPropertyIri) -> bool {
		*self == MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_IRI_HTTP
			|| *self == MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct MissionCoveragePrioritiesPolicyPropertyIriOrLabel;
impl PartialEq<&str> for MissionCoveragePrioritiesPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MissionCoveragePrioritiesPolicyPropertyIri
			|| *other == MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<MissionCoveragePrioritiesPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MissionCoveragePrioritiesPolicyPropertyIriOrLabel) -> bool {
		*self == MissionCoveragePrioritiesPolicyPropertyIri
			|| *self == MISSION_COVERAGE_PRIORITIES_POLICY_PROPERTY_LABEL
	}
}
