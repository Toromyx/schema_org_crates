/// <https://schema.org/awayTeam>
pub const AWAY_TEAM_PROPERTY_IRI_HTTP: &str = "http://schema.org/awayTeam";
/// <https://schema.org/awayTeam>
pub const AWAY_TEAM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/awayTeam";
/// <https://schema.org/awayTeam>
pub const AWAY_TEAM_PROPERTY_LABEL: &str = "awayTeam";
pub struct AwayTeamPropertyIri;
impl PartialEq<&str> for AwayTeamPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AWAY_TEAM_PROPERTY_IRI_HTTP || *other == AWAY_TEAM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AwayTeamPropertyIri> for &str {
	fn eq(&self, other: &AwayTeamPropertyIri) -> bool {
		*self == AWAY_TEAM_PROPERTY_IRI_HTTP || *self == AWAY_TEAM_PROPERTY_IRI_HTTPS
	}
}
pub struct AwayTeamPropertyIriOrLabel;
impl PartialEq<&str> for AwayTeamPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AwayTeamPropertyIri || *other == AWAY_TEAM_PROPERTY_LABEL
	}
}
impl PartialEq<AwayTeamPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AwayTeamPropertyIriOrLabel) -> bool {
		*self == AwayTeamPropertyIri || *self == AWAY_TEAM_PROPERTY_LABEL
	}
}
