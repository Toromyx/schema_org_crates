/// <https://schema.org/interactionStatistic>
pub const INTERACTION_STATISTIC_PROPERTY_IRI_HTTP: &str = "http://schema.org/interactionStatistic";
/// <https://schema.org/interactionStatistic>
pub const INTERACTION_STATISTIC_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/interactionStatistic";
/// <https://schema.org/interactionStatistic>
pub const INTERACTION_STATISTIC_PROPERTY_LABEL: &str = "interactionStatistic";
pub struct InteractionStatisticPropertyIri;
impl PartialEq<&str> for InteractionStatisticPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERACTION_STATISTIC_PROPERTY_IRI_HTTP
			|| *other == INTERACTION_STATISTIC_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InteractionStatisticPropertyIri> for &str {
	fn eq(&self, other: &InteractionStatisticPropertyIri) -> bool {
		*self == INTERACTION_STATISTIC_PROPERTY_IRI_HTTP
			|| *self == INTERACTION_STATISTIC_PROPERTY_IRI_HTTPS
	}
}
pub struct InteractionStatisticPropertyIriOrLabel;
impl PartialEq<&str> for InteractionStatisticPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InteractionStatisticPropertyIri || *other == INTERACTION_STATISTIC_PROPERTY_LABEL
	}
}
impl PartialEq<InteractionStatisticPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InteractionStatisticPropertyIriOrLabel) -> bool {
		*self == InteractionStatisticPropertyIri || *self == INTERACTION_STATISTIC_PROPERTY_LABEL
	}
}
