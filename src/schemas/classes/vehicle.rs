use super::*;
/// <https://schema.org/Vehicle>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Vehicle {
	#[cfg(any(
		any(
			feature = "acceleration-time-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#acceleration_time: Vec<AccelerationTimeProperty>,
	#[cfg(any(
		any(
			feature = "additional-property-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	#[cfg(any(
		any(
			feature = "additional-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	#[cfg(any(
		any(
			feature = "aggregate-rating-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	#[cfg(any(
		any(
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(feature = "asin-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#asin: Vec<AsinProperty>,
	#[cfg(any(
		any(
			feature = "audience-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#audience: Vec<AudienceProperty>,
	#[cfg(any(
		any(feature = "award-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#award: Vec<AwardProperty>,
	#[cfg(any(
		any(feature = "awards-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#awards: Vec<AwardsProperty>,
	#[cfg(any(
		any(feature = "body-type-property-schema", feature = "auto-schema-section"),
		doc
	))]
	pub r#body_type: Vec<BodyTypeProperty>,
	#[cfg(any(
		any(feature = "brand-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#brand: Vec<BrandProperty>,
	#[cfg(any(
		any(
			feature = "call-sign-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#call_sign: Vec<CallSignProperty>,
	#[cfg(any(
		any(
			feature = "cargo-volume-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#cargo_volume: Vec<CargoVolumeProperty>,
	#[cfg(any(
		any(
			feature = "category-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#category: Vec<CategoryProperty>,
	#[cfg(any(
		any(feature = "color-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#color: Vec<ColorProperty>,
	#[cfg(any(
		any(
			feature = "country-of-assembly-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#country_of_assembly: Vec<CountryOfAssemblyProperty>,
	#[cfg(any(
		any(
			feature = "country-of-last-processing-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#country_of_last_processing: Vec<CountryOfLastProcessingProperty>,
	#[cfg(any(
		any(
			feature = "country-of-origin-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#country_of_origin: Vec<CountryOfOriginProperty>,
	#[cfg(any(
		any(
			feature = "date-vehicle-first-registered-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#date_vehicle_first_registered: Vec<DateVehicleFirstRegisteredProperty>,
	#[cfg(any(
		any(feature = "depth-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#depth: Vec<DepthProperty>,
	#[cfg(any(
		any(
			feature = "description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#description: Vec<DescriptionProperty>,
	#[cfg(any(
		any(
			feature = "disambiguating-description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	#[cfg(any(
		any(
			feature = "drive-wheel-configuration-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#drive_wheel_configuration: Vec<DriveWheelConfigurationProperty>,
	#[cfg(any(
		any(
			feature = "emissions-co-2-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#emissions_co_2: Vec<EmissionsCo2Property>,
	#[cfg(any(
		any(
			feature = "fuel-capacity-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#fuel_capacity: Vec<FuelCapacityProperty>,
	#[cfg(any(
		any(
			feature = "fuel-consumption-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#fuel_consumption: Vec<FuelConsumptionProperty>,
	#[cfg(any(
		any(
			feature = "fuel-efficiency-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#fuel_efficiency: Vec<FuelEfficiencyProperty>,
	#[cfg(any(
		any(
			feature = "fuel-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#fuel_type: Vec<FuelTypeProperty>,
	#[cfg(any(
		any(
			feature = "funding-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#funding: Vec<FundingProperty>,
	#[cfg(any(
		any(feature = "gtin-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#gtin: Vec<GtinProperty>,
	#[cfg(any(
		any(
			feature = "gtin-12-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#gtin_12: Vec<Gtin12Property>,
	#[cfg(any(
		any(
			feature = "gtin-13-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#gtin_13: Vec<Gtin13Property>,
	#[cfg(any(
		any(
			feature = "gtin-14-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#gtin_14: Vec<Gtin14Property>,
	#[cfg(any(
		any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#gtin_8: Vec<Gtin8Property>,
	#[cfg(any(
		any(
			feature = "has-adult-consideration-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
	#[cfg(any(
		any(
			feature = "has-energy-consumption-details-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_energy_consumption_details: Vec<HasEnergyConsumptionDetailsProperty>,
	#[cfg(any(
		any(
			feature = "has-measurement-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_measurement: Vec<HasMeasurementProperty>,
	#[cfg(any(
		any(
			feature = "has-merchant-return-policy-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
	#[cfg(any(
		any(
			feature = "has-product-return-policy-property-schema",
			feature = "attic-schema-section"
		),
		doc
	))]
	pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
	#[cfg(any(
		any(feature = "height-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#height: Vec<HeightProperty>,
	#[cfg(any(
		any(
			feature = "identifier-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#identifier: Vec<IdentifierProperty>,
	#[cfg(any(
		any(feature = "image-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#image: Vec<ImageProperty>,
	#[cfg(any(
		any(
			feature = "in-product-group-with-id-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#in_product_group_with_id: Vec<InProductGroupWithIdProperty>,
	#[cfg(any(
		any(
			feature = "is-accessory-or-spare-part-for-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_accessory_or_spare_part_for: Vec<IsAccessoryOrSparePartForProperty>,
	#[cfg(any(
		any(
			feature = "is-consumable-for-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_consumable_for: Vec<IsConsumableForProperty>,
	#[cfg(any(
		any(
			feature = "is-family-friendly-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
	#[cfg(any(
		any(
			feature = "is-related-to-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_related_to: Vec<IsRelatedToProperty>,
	#[cfg(any(
		any(
			feature = "is-similar-to-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_similar_to: Vec<IsSimilarToProperty>,
	#[cfg(any(
		any(
			feature = "is-variant-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_variant_of: Vec<IsVariantOfProperty>,
	#[cfg(any(
		any(
			feature = "item-condition-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#item_condition: Vec<ItemConditionProperty>,
	#[cfg(any(
		any(
			feature = "keywords-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#keywords: Vec<KeywordsProperty>,
	#[cfg(any(
		any(
			feature = "known-vehicle-damages-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#known_vehicle_damages: Vec<KnownVehicleDamagesProperty>,
	#[cfg(any(
		any(feature = "logo-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#logo: Vec<LogoProperty>,
	#[cfg(any(
		any(
			feature = "main-entity-of-page-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	#[cfg(any(
		any(
			feature = "manufacturer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#manufacturer: Vec<ManufacturerProperty>,
	#[cfg(any(
		any(
			feature = "material-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#material: Vec<MaterialProperty>,
	#[cfg(any(
		any(
			feature = "meets-emission-standard-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#meets_emission_standard: Vec<MeetsEmissionStandardProperty>,
	#[cfg(any(
		any(
			feature = "mileage-from-odometer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#mileage_from_odometer: Vec<MileageFromOdometerProperty>,
	#[cfg(any(
		any(
			feature = "mobile-url-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#mobile_url: Vec<MobileUrlProperty>,
	#[cfg(any(
		any(feature = "model-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#model: Vec<ModelProperty>,
	#[cfg(any(
		any(
			feature = "model-date-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#model_date: Vec<ModelDateProperty>,
	#[cfg(any(
		any(feature = "mpn-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#mpn: Vec<MpnProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "negative-notes-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#negative_notes: Vec<NegativeNotesProperty>,
	#[cfg(any(
		any(feature = "nsn-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#nsn: Vec<NsnProperty>,
	#[cfg(any(
		any(
			feature = "number-of-airbags-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#number_of_airbags: Vec<NumberOfAirbagsProperty>,
	#[cfg(any(
		any(
			feature = "number-of-axles-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#number_of_axles: Vec<NumberOfAxlesProperty>,
	#[cfg(any(
		any(
			feature = "number-of-doors-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#number_of_doors: Vec<NumberOfDoorsProperty>,
	#[cfg(any(
		any(
			feature = "number-of-forward-gears-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#number_of_forward_gears: Vec<NumberOfForwardGearsProperty>,
	#[cfg(any(
		any(
			feature = "number-of-previous-owners-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#number_of_previous_owners: Vec<NumberOfPreviousOwnersProperty>,
	#[cfg(any(
		any(feature = "offers-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#offers: Vec<OffersProperty>,
	#[cfg(any(
		any(
			feature = "pattern-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#pattern: Vec<PatternProperty>,
	#[cfg(any(
		any(feature = "payload-property-schema", feature = "auto-schema-section"),
		doc
	))]
	pub r#payload: Vec<PayloadProperty>,
	#[cfg(any(
		any(
			feature = "positive-notes-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#positive_notes: Vec<PositiveNotesProperty>,
	#[cfg(any(
		any(
			feature = "potential-action-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#potential_action: Vec<PotentialActionProperty>,
	#[cfg(any(
		any(
			feature = "product-id-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#product_id: Vec<ProductIdProperty>,
	#[cfg(any(
		any(
			feature = "production-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#production_date: Vec<ProductionDateProperty>,
	#[cfg(any(
		any(
			feature = "purchase-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#purchase_date: Vec<PurchaseDateProperty>,
	#[cfg(any(
		any(
			feature = "release-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#release_date: Vec<ReleaseDateProperty>,
	#[cfg(any(
		any(feature = "review-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#review: Vec<ReviewProperty>,
	#[cfg(any(
		any(
			feature = "reviews-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#reviews: Vec<ReviewsProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(
			feature = "seating-capacity-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#seating_capacity: Vec<SeatingCapacityProperty>,
	#[cfg(any(
		any(feature = "size-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#size: Vec<SizeProperty>,
	#[cfg(any(
		any(feature = "sku-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#sku: Vec<SkuProperty>,
	#[cfg(any(
		any(feature = "slogan-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#slogan: Vec<SloganProperty>,
	#[cfg(any(
		any(feature = "speed-property-schema", feature = "auto-schema-section"),
		doc
	))]
	pub r#speed: Vec<SpeedProperty>,
	#[cfg(any(
		any(
			feature = "steering-position-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#steering_position: Vec<SteeringPositionProperty>,
	#[cfg(any(
		any(
			feature = "stupid-property-property-schema",
			feature = "attic-schema-section"
		),
		doc
	))]
	pub r#stupid_property: Vec<StupidPropertyProperty>,
	#[cfg(any(
		any(
			feature = "subject-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#subject_of: Vec<SubjectOfProperty>,
	#[cfg(any(
		any(
			feature = "tongue-weight-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#tongue_weight: Vec<TongueWeightProperty>,
	#[cfg(any(
		any(
			feature = "trailer-weight-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#trailer_weight: Vec<TrailerWeightProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-configuration-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#vehicle_configuration: Vec<VehicleConfigurationProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-engine-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#vehicle_engine: Vec<VehicleEngineProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-identification-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#vehicle_identification_number: Vec<VehicleIdentificationNumberProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-interior-color-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#vehicle_interior_color: Vec<VehicleInteriorColorProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-interior-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#vehicle_interior_type: Vec<VehicleInteriorTypeProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-model-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#vehicle_model_date: Vec<VehicleModelDateProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-seating-capacity-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#vehicle_seating_capacity: Vec<VehicleSeatingCapacityProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-special-usage-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#vehicle_special_usage: Vec<VehicleSpecialUsageProperty>,
	#[cfg(any(
		any(
			feature = "vehicle-transmission-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#vehicle_transmission: Vec<VehicleTransmissionProperty>,
	#[cfg(any(
		any(feature = "weight-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#weight: Vec<WeightProperty>,
	#[cfg(any(
		any(
			feature = "weight-total-property-schema",
			feature = "auto-schema-section"
		),
		doc
	))]
	pub r#weight_total: Vec<WeightTotalProperty>,
	#[cfg(any(
		any(feature = "wheelbase-property-schema", feature = "auto-schema-section"),
		doc
	))]
	pub r#wheelbase: Vec<WheelbaseProperty>,
	#[cfg(any(
		any(feature = "width-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#width: Vec<WidthProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Vehicle {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "acceleration-time-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#acceleration_time) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#additional_property) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#additional_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#aggregate_rating) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#alternate_name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "asin-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#asin) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#audience) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "award-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#award) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "awards-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#awards) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "body-type-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#body_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "brand-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#brand) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "call-sign-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#call_sign) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cargo-volume-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cargo_volume) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "category-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#category) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "color-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#color) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "country-of-assembly-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#country_of_assembly) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "country-of-last-processing-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#country_of_last_processing) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#country_of_origin) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "date-vehicle-first-registered-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#date_vehicle_first_registered) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "depth-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#depth) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#description) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#disambiguating_description) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "drive-wheel-configuration-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#drive_wheel_configuration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "emissions-co-2-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#emissions_co_2) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "fuel-capacity-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#fuel_capacity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "fuel-consumption-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#fuel_consumption) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "fuel-efficiency-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#fuel_efficiency) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "fuel-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#fuel_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#funding) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-12-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin_12) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-13-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin_13) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-14-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin_14) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gtin-8-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gtin_8) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-adult-consideration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_adult_consideration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-energy-consumption-details-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_energy_consumption_details) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_measurement) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_merchant_return_policy) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_product_return_policy) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "height-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#height) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#identifier) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "image-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#image) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "in-product-group-with-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#in_product_group_with_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-accessory-or-spare-part-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_accessory_or_spare_part_for) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-consumable-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_consumable_for) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_family_friendly) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_related_to) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-similar-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_similar_to) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-variant-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_variant_of) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "item-condition-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#item_condition) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#keywords) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "known-vehicle-damages-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#known_vehicle_damages) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "logo-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#logo) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#main_entity_of_page) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "manufacturer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#manufacturer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#material) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "meets-emission-standard-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#meets_emission_standard) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "mileage-from-odometer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#mileage_from_odometer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "mobile-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#mobile_url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "model-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#model) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "model-date-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#model_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "mpn-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#mpn) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "negative-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#negative_notes) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "nsn-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#nsn) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-airbags-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_airbags) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-axles-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_axles) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-doors-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_doors) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-forward-gears-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_forward_gears) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-previous-owners-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_previous_owners) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "offers-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#offers) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#pattern) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "payload-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#payload) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "positive-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#positive_notes) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#potential_action) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "product-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#product_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "production-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#production_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "purchase-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#purchase_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "release-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#release_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "review-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#review) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#reviews) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#same_as) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "seating-capacity-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#seating_capacity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "size-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#size) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sku-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sku) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "slogan-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#slogan) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "speed-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#speed) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "steering-position-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#steering_position) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "stupid-property-property-schema",
						feature = "attic-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#stupid_property) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#subject_of) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "tongue-weight-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#tongue_weight) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "trailer-weight-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#trailer_weight) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-configuration-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_configuration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-engine-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_engine) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-identification-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_identification_number) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-interior-color-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_interior_color) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-interior-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_interior_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-model-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_model_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-seating-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_seating_capacity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-special-usage-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_special_usage) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "vehicle-transmission-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vehicle_transmission) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "weight-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#weight) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "weight-total-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#weight_total) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "wheelbase-property-schema",
						feature = "auto-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#wheelbase) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "width-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#width) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Vehicle", len)?;
			#[cfg(any(
				any(
					feature = "acceleration-time-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#acceleration_time) {
				serialize_struct.serialize_field("accelerationTime", {
					struct SerializeWith<'a>(&'a Vec<AccelerationTimeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#acceleration_time)
				})?;
			} else {
				serialize_struct.skip_field("accelerationTime")?;
			}
			#[cfg(any(
				any(
					feature = "additional-property-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#additional_property) {
				serialize_struct.serialize_field("additionalProperty", {
					struct SerializeWith<'a>(&'a Vec<AdditionalPropertyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#additional_property)
				})?;
			} else {
				serialize_struct.skip_field("additionalProperty")?;
			}
			#[cfg(any(
				any(
					feature = "additional-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#additional_type) {
				serialize_struct.serialize_field("additionalType", {
					struct SerializeWith<'a>(&'a Vec<AdditionalTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#additional_type)
				})?;
			} else {
				serialize_struct.skip_field("additionalType")?;
			}
			#[cfg(any(
				any(
					feature = "aggregate-rating-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#aggregate_rating) {
				serialize_struct.serialize_field("aggregateRating", {
					struct SerializeWith<'a>(&'a Vec<AggregateRatingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#aggregate_rating)
				})?;
			} else {
				serialize_struct.skip_field("aggregateRating")?;
			}
			#[cfg(any(
				any(
					feature = "alternate-name-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#alternate_name) {
				serialize_struct.serialize_field("alternateName", {
					struct SerializeWith<'a>(&'a Vec<AlternateNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#alternate_name)
				})?;
			} else {
				serialize_struct.skip_field("alternateName")?;
			}
			#[cfg(any(
				any(feature = "asin-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#asin) {
				serialize_struct.serialize_field("asin", {
					struct SerializeWith<'a>(&'a Vec<AsinProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#asin)
				})?;
			} else {
				serialize_struct.skip_field("asin")?;
			}
			#[cfg(any(
				any(
					feature = "audience-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#audience) {
				serialize_struct.serialize_field("audience", {
					struct SerializeWith<'a>(&'a Vec<AudienceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#audience)
				})?;
			} else {
				serialize_struct.skip_field("audience")?;
			}
			#[cfg(any(
				any(feature = "award-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#award) {
				serialize_struct.serialize_field("award", {
					struct SerializeWith<'a>(&'a Vec<AwardProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#award)
				})?;
			} else {
				serialize_struct.skip_field("award")?;
			}
			#[cfg(any(
				any(feature = "awards-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#awards) {
				serialize_struct.serialize_field("awards", {
					struct SerializeWith<'a>(&'a Vec<AwardsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#awards)
				})?;
			} else {
				serialize_struct.skip_field("awards")?;
			}
			#[cfg(any(
				any(feature = "body-type-property-schema", feature = "auto-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#body_type) {
				serialize_struct.serialize_field("bodyType", {
					struct SerializeWith<'a>(&'a Vec<BodyTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#body_type)
				})?;
			} else {
				serialize_struct.skip_field("bodyType")?;
			}
			#[cfg(any(
				any(feature = "brand-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#brand) {
				serialize_struct.serialize_field("brand", {
					struct SerializeWith<'a>(&'a Vec<BrandProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#brand)
				})?;
			} else {
				serialize_struct.skip_field("brand")?;
			}
			#[cfg(any(
				any(
					feature = "call-sign-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#call_sign) {
				serialize_struct.serialize_field("callSign", {
					struct SerializeWith<'a>(&'a Vec<CallSignProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#call_sign)
				})?;
			} else {
				serialize_struct.skip_field("callSign")?;
			}
			#[cfg(any(
				any(
					feature = "cargo-volume-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#cargo_volume) {
				serialize_struct.serialize_field("cargoVolume", {
					struct SerializeWith<'a>(&'a Vec<CargoVolumeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cargo_volume)
				})?;
			} else {
				serialize_struct.skip_field("cargoVolume")?;
			}
			#[cfg(any(
				any(
					feature = "category-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#category) {
				serialize_struct.serialize_field("category", {
					struct SerializeWith<'a>(&'a Vec<CategoryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#category)
				})?;
			} else {
				serialize_struct.skip_field("category")?;
			}
			#[cfg(any(
				any(feature = "color-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#color) {
				serialize_struct.serialize_field("color", {
					struct SerializeWith<'a>(&'a Vec<ColorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#color)
				})?;
			} else {
				serialize_struct.skip_field("color")?;
			}
			#[cfg(any(
				any(
					feature = "country-of-assembly-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#country_of_assembly) {
				serialize_struct.serialize_field("countryOfAssembly", {
					struct SerializeWith<'a>(&'a Vec<CountryOfAssemblyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#country_of_assembly)
				})?;
			} else {
				serialize_struct.skip_field("countryOfAssembly")?;
			}
			#[cfg(any(
				any(
					feature = "country-of-last-processing-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#country_of_last_processing) {
				serialize_struct.serialize_field("countryOfLastProcessing", {
					struct SerializeWith<'a>(&'a Vec<CountryOfLastProcessingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#country_of_last_processing)
				})?;
			} else {
				serialize_struct.skip_field("countryOfLastProcessing")?;
			}
			#[cfg(any(
				any(
					feature = "country-of-origin-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#country_of_origin) {
				serialize_struct.serialize_field("countryOfOrigin", {
					struct SerializeWith<'a>(&'a Vec<CountryOfOriginProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#country_of_origin)
				})?;
			} else {
				serialize_struct.skip_field("countryOfOrigin")?;
			}
			#[cfg(any(
				any(
					feature = "date-vehicle-first-registered-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#date_vehicle_first_registered) {
				serialize_struct.serialize_field("dateVehicleFirstRegistered", {
					struct SerializeWith<'a>(&'a Vec<DateVehicleFirstRegisteredProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#date_vehicle_first_registered)
				})?;
			} else {
				serialize_struct.skip_field("dateVehicleFirstRegistered")?;
			}
			#[cfg(any(
				any(feature = "depth-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#depth) {
				serialize_struct.serialize_field("depth", {
					struct SerializeWith<'a>(&'a Vec<DepthProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#depth)
				})?;
			} else {
				serialize_struct.skip_field("depth")?;
			}
			#[cfg(any(
				any(
					feature = "description-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#description) {
				serialize_struct.serialize_field("description", {
					struct SerializeWith<'a>(&'a Vec<DescriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#description)
				})?;
			} else {
				serialize_struct.skip_field("description")?;
			}
			#[cfg(any(
				any(
					feature = "disambiguating-description-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#disambiguating_description) {
				serialize_struct.serialize_field("disambiguatingDescription", {
					struct SerializeWith<'a>(&'a Vec<DisambiguatingDescriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#disambiguating_description)
				})?;
			} else {
				serialize_struct.skip_field("disambiguatingDescription")?;
			}
			#[cfg(any(
				any(
					feature = "drive-wheel-configuration-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#drive_wheel_configuration) {
				serialize_struct.serialize_field("driveWheelConfiguration", {
					struct SerializeWith<'a>(&'a Vec<DriveWheelConfigurationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#drive_wheel_configuration)
				})?;
			} else {
				serialize_struct.skip_field("driveWheelConfiguration")?;
			}
			#[cfg(any(
				any(
					feature = "emissions-co-2-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#emissions_co_2) {
				serialize_struct.serialize_field("emissionsCO2", {
					struct SerializeWith<'a>(&'a Vec<EmissionsCo2Property>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#emissions_co_2)
				})?;
			} else {
				serialize_struct.skip_field("emissionsCO2")?;
			}
			#[cfg(any(
				any(
					feature = "fuel-capacity-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#fuel_capacity) {
				serialize_struct.serialize_field("fuelCapacity", {
					struct SerializeWith<'a>(&'a Vec<FuelCapacityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fuel_capacity)
				})?;
			} else {
				serialize_struct.skip_field("fuelCapacity")?;
			}
			#[cfg(any(
				any(
					feature = "fuel-consumption-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#fuel_consumption) {
				serialize_struct.serialize_field("fuelConsumption", {
					struct SerializeWith<'a>(&'a Vec<FuelConsumptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fuel_consumption)
				})?;
			} else {
				serialize_struct.skip_field("fuelConsumption")?;
			}
			#[cfg(any(
				any(
					feature = "fuel-efficiency-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#fuel_efficiency) {
				serialize_struct.serialize_field("fuelEfficiency", {
					struct SerializeWith<'a>(&'a Vec<FuelEfficiencyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fuel_efficiency)
				})?;
			} else {
				serialize_struct.skip_field("fuelEfficiency")?;
			}
			#[cfg(any(
				any(
					feature = "fuel-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#fuel_type) {
				serialize_struct.serialize_field("fuelType", {
					struct SerializeWith<'a>(&'a Vec<FuelTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fuel_type)
				})?;
			} else {
				serialize_struct.skip_field("fuelType")?;
			}
			#[cfg(any(
				any(
					feature = "funding-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#funding) {
				serialize_struct.serialize_field("funding", {
					struct SerializeWith<'a>(&'a Vec<FundingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#funding)
				})?;
			} else {
				serialize_struct.skip_field("funding")?;
			}
			#[cfg(any(
				any(feature = "gtin-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin) {
				serialize_struct.serialize_field("gtin", {
					struct SerializeWith<'a>(&'a Vec<GtinProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#gtin)
				})?;
			} else {
				serialize_struct.skip_field("gtin")?;
			}
			#[cfg(any(
				any(
					feature = "gtin-12-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin_12) {
				serialize_struct.serialize_field("gtin12", {
					struct SerializeWith<'a>(&'a Vec<Gtin12Property>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#gtin_12)
				})?;
			} else {
				serialize_struct.skip_field("gtin12")?;
			}
			#[cfg(any(
				any(
					feature = "gtin-13-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin_13) {
				serialize_struct.serialize_field("gtin13", {
					struct SerializeWith<'a>(&'a Vec<Gtin13Property>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#gtin_13)
				})?;
			} else {
				serialize_struct.skip_field("gtin13")?;
			}
			#[cfg(any(
				any(
					feature = "gtin-14-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin_14) {
				serialize_struct.serialize_field("gtin14", {
					struct SerializeWith<'a>(&'a Vec<Gtin14Property>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#gtin_14)
				})?;
			} else {
				serialize_struct.skip_field("gtin14")?;
			}
			#[cfg(any(
				any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#gtin_8) {
				serialize_struct.serialize_field("gtin8", {
					struct SerializeWith<'a>(&'a Vec<Gtin8Property>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#gtin_8)
				})?;
			} else {
				serialize_struct.skip_field("gtin8")?;
			}
			#[cfg(any(
				any(
					feature = "has-adult-consideration-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_adult_consideration) {
				serialize_struct.serialize_field("hasAdultConsideration", {
					struct SerializeWith<'a>(&'a Vec<HasAdultConsiderationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_adult_consideration)
				})?;
			} else {
				serialize_struct.skip_field("hasAdultConsideration")?;
			}
			#[cfg(any(
				any(
					feature = "has-energy-consumption-details-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_energy_consumption_details) {
				serialize_struct.serialize_field("hasEnergyConsumptionDetails", {
					struct SerializeWith<'a>(&'a Vec<HasEnergyConsumptionDetailsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_energy_consumption_details)
				})?;
			} else {
				serialize_struct.skip_field("hasEnergyConsumptionDetails")?;
			}
			#[cfg(any(
				any(
					feature = "has-measurement-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_measurement) {
				serialize_struct.serialize_field("hasMeasurement", {
					struct SerializeWith<'a>(&'a Vec<HasMeasurementProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_measurement)
				})?;
			} else {
				serialize_struct.skip_field("hasMeasurement")?;
			}
			#[cfg(any(
				any(
					feature = "has-merchant-return-policy-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_merchant_return_policy) {
				serialize_struct.serialize_field("hasMerchantReturnPolicy", {
					struct SerializeWith<'a>(&'a Vec<HasMerchantReturnPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_merchant_return_policy)
				})?;
			} else {
				serialize_struct.skip_field("hasMerchantReturnPolicy")?;
			}
			#[cfg(any(
				any(
					feature = "has-product-return-policy-property-schema",
					feature = "attic-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_product_return_policy) {
				serialize_struct.serialize_field("hasProductReturnPolicy", {
					struct SerializeWith<'a>(&'a Vec<HasProductReturnPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_product_return_policy)
				})?;
			} else {
				serialize_struct.skip_field("hasProductReturnPolicy")?;
			}
			#[cfg(any(
				any(feature = "height-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#height) {
				serialize_struct.serialize_field("height", {
					struct SerializeWith<'a>(&'a Vec<HeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#height)
				})?;
			} else {
				serialize_struct.skip_field("height")?;
			}
			#[cfg(any(
				any(
					feature = "identifier-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#identifier) {
				serialize_struct.serialize_field("identifier", {
					struct SerializeWith<'a>(&'a Vec<IdentifierProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#identifier)
				})?;
			} else {
				serialize_struct.skip_field("identifier")?;
			}
			#[cfg(any(
				any(feature = "image-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#image) {
				serialize_struct.serialize_field("image", {
					struct SerializeWith<'a>(&'a Vec<ImageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#image)
				})?;
			} else {
				serialize_struct.skip_field("image")?;
			}
			#[cfg(any(
				any(
					feature = "in-product-group-with-id-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#in_product_group_with_id) {
				serialize_struct.serialize_field("inProductGroupWithID", {
					struct SerializeWith<'a>(&'a Vec<InProductGroupWithIdProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#in_product_group_with_id)
				})?;
			} else {
				serialize_struct.skip_field("inProductGroupWithID")?;
			}
			#[cfg(any(
				any(
					feature = "is-accessory-or-spare-part-for-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_accessory_or_spare_part_for) {
				serialize_struct.serialize_field("isAccessoryOrSparePartFor", {
					struct SerializeWith<'a>(&'a Vec<IsAccessoryOrSparePartForProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_accessory_or_spare_part_for)
				})?;
			} else {
				serialize_struct.skip_field("isAccessoryOrSparePartFor")?;
			}
			#[cfg(any(
				any(
					feature = "is-consumable-for-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_consumable_for) {
				serialize_struct.serialize_field("isConsumableFor", {
					struct SerializeWith<'a>(&'a Vec<IsConsumableForProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_consumable_for)
				})?;
			} else {
				serialize_struct.skip_field("isConsumableFor")?;
			}
			#[cfg(any(
				any(
					feature = "is-family-friendly-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_family_friendly) {
				serialize_struct.serialize_field("isFamilyFriendly", {
					struct SerializeWith<'a>(&'a Vec<IsFamilyFriendlyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_family_friendly)
				})?;
			} else {
				serialize_struct.skip_field("isFamilyFriendly")?;
			}
			#[cfg(any(
				any(
					feature = "is-related-to-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_related_to) {
				serialize_struct.serialize_field("isRelatedTo", {
					struct SerializeWith<'a>(&'a Vec<IsRelatedToProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_related_to)
				})?;
			} else {
				serialize_struct.skip_field("isRelatedTo")?;
			}
			#[cfg(any(
				any(
					feature = "is-similar-to-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_similar_to) {
				serialize_struct.serialize_field("isSimilarTo", {
					struct SerializeWith<'a>(&'a Vec<IsSimilarToProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_similar_to)
				})?;
			} else {
				serialize_struct.skip_field("isSimilarTo")?;
			}
			#[cfg(any(
				any(
					feature = "is-variant-of-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#is_variant_of) {
				serialize_struct.serialize_field("isVariantOf", {
					struct SerializeWith<'a>(&'a Vec<IsVariantOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_variant_of)
				})?;
			} else {
				serialize_struct.skip_field("isVariantOf")?;
			}
			#[cfg(any(
				any(
					feature = "item-condition-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#item_condition) {
				serialize_struct.serialize_field("itemCondition", {
					struct SerializeWith<'a>(&'a Vec<ItemConditionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#item_condition)
				})?;
			} else {
				serialize_struct.skip_field("itemCondition")?;
			}
			#[cfg(any(
				any(
					feature = "keywords-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#keywords) {
				serialize_struct.serialize_field("keywords", {
					struct SerializeWith<'a>(&'a Vec<KeywordsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#keywords)
				})?;
			} else {
				serialize_struct.skip_field("keywords")?;
			}
			#[cfg(any(
				any(
					feature = "known-vehicle-damages-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#known_vehicle_damages) {
				serialize_struct.serialize_field("knownVehicleDamages", {
					struct SerializeWith<'a>(&'a Vec<KnownVehicleDamagesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#known_vehicle_damages)
				})?;
			} else {
				serialize_struct.skip_field("knownVehicleDamages")?;
			}
			#[cfg(any(
				any(feature = "logo-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#logo) {
				serialize_struct.serialize_field("logo", {
					struct SerializeWith<'a>(&'a Vec<LogoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#logo)
				})?;
			} else {
				serialize_struct.skip_field("logo")?;
			}
			#[cfg(any(
				any(
					feature = "main-entity-of-page-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#main_entity_of_page) {
				serialize_struct.serialize_field("mainEntityOfPage", {
					struct SerializeWith<'a>(&'a Vec<MainEntityOfPageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#main_entity_of_page)
				})?;
			} else {
				serialize_struct.skip_field("mainEntityOfPage")?;
			}
			#[cfg(any(
				any(
					feature = "manufacturer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#manufacturer) {
				serialize_struct.serialize_field("manufacturer", {
					struct SerializeWith<'a>(&'a Vec<ManufacturerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#manufacturer)
				})?;
			} else {
				serialize_struct.skip_field("manufacturer")?;
			}
			#[cfg(any(
				any(
					feature = "material-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#material) {
				serialize_struct.serialize_field("material", {
					struct SerializeWith<'a>(&'a Vec<MaterialProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#material)
				})?;
			} else {
				serialize_struct.skip_field("material")?;
			}
			#[cfg(any(
				any(
					feature = "meets-emission-standard-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#meets_emission_standard) {
				serialize_struct.serialize_field("meetsEmissionStandard", {
					struct SerializeWith<'a>(&'a Vec<MeetsEmissionStandardProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#meets_emission_standard)
				})?;
			} else {
				serialize_struct.skip_field("meetsEmissionStandard")?;
			}
			#[cfg(any(
				any(
					feature = "mileage-from-odometer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#mileage_from_odometer) {
				serialize_struct.serialize_field("mileageFromOdometer", {
					struct SerializeWith<'a>(&'a Vec<MileageFromOdometerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#mileage_from_odometer)
				})?;
			} else {
				serialize_struct.skip_field("mileageFromOdometer")?;
			}
			#[cfg(any(
				any(
					feature = "mobile-url-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#mobile_url) {
				serialize_struct.serialize_field("mobileUrl", {
					struct SerializeWith<'a>(&'a Vec<MobileUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#mobile_url)
				})?;
			} else {
				serialize_struct.skip_field("mobileUrl")?;
			}
			#[cfg(any(
				any(feature = "model-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#model) {
				serialize_struct.serialize_field("model", {
					struct SerializeWith<'a>(&'a Vec<ModelProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#model)
				})?;
			} else {
				serialize_struct.skip_field("model")?;
			}
			#[cfg(any(
				any(
					feature = "model-date-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#model_date) {
				serialize_struct.serialize_field("modelDate", {
					struct SerializeWith<'a>(&'a Vec<ModelDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#model_date)
				})?;
			} else {
				serialize_struct.skip_field("modelDate")?;
			}
			#[cfg(any(
				any(feature = "mpn-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#mpn) {
				serialize_struct.serialize_field("mpn", {
					struct SerializeWith<'a>(&'a Vec<MpnProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#mpn)
				})?;
			} else {
				serialize_struct.skip_field("mpn")?;
			}
			#[cfg(any(
				any(feature = "name-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#name) {
				serialize_struct.serialize_field("name", {
					struct SerializeWith<'a>(&'a Vec<NameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#name)
				})?;
			} else {
				serialize_struct.skip_field("name")?;
			}
			#[cfg(any(
				any(
					feature = "negative-notes-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#negative_notes) {
				serialize_struct.serialize_field("negativeNotes", {
					struct SerializeWith<'a>(&'a Vec<NegativeNotesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#negative_notes)
				})?;
			} else {
				serialize_struct.skip_field("negativeNotes")?;
			}
			#[cfg(any(
				any(feature = "nsn-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#nsn) {
				serialize_struct.serialize_field("nsn", {
					struct SerializeWith<'a>(&'a Vec<NsnProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#nsn)
				})?;
			} else {
				serialize_struct.skip_field("nsn")?;
			}
			#[cfg(any(
				any(
					feature = "number-of-airbags-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#number_of_airbags) {
				serialize_struct.serialize_field("numberOfAirbags", {
					struct SerializeWith<'a>(&'a Vec<NumberOfAirbagsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_airbags)
				})?;
			} else {
				serialize_struct.skip_field("numberOfAirbags")?;
			}
			#[cfg(any(
				any(
					feature = "number-of-axles-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#number_of_axles) {
				serialize_struct.serialize_field("numberOfAxles", {
					struct SerializeWith<'a>(&'a Vec<NumberOfAxlesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_axles)
				})?;
			} else {
				serialize_struct.skip_field("numberOfAxles")?;
			}
			#[cfg(any(
				any(
					feature = "number-of-doors-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#number_of_doors) {
				serialize_struct.serialize_field("numberOfDoors", {
					struct SerializeWith<'a>(&'a Vec<NumberOfDoorsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_doors)
				})?;
			} else {
				serialize_struct.skip_field("numberOfDoors")?;
			}
			#[cfg(any(
				any(
					feature = "number-of-forward-gears-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#number_of_forward_gears) {
				serialize_struct.serialize_field("numberOfForwardGears", {
					struct SerializeWith<'a>(&'a Vec<NumberOfForwardGearsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_forward_gears)
				})?;
			} else {
				serialize_struct.skip_field("numberOfForwardGears")?;
			}
			#[cfg(any(
				any(
					feature = "number-of-previous-owners-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#number_of_previous_owners) {
				serialize_struct.serialize_field("numberOfPreviousOwners", {
					struct SerializeWith<'a>(&'a Vec<NumberOfPreviousOwnersProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_previous_owners)
				})?;
			} else {
				serialize_struct.skip_field("numberOfPreviousOwners")?;
			}
			#[cfg(any(
				any(feature = "offers-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#offers) {
				serialize_struct.serialize_field("offers", {
					struct SerializeWith<'a>(&'a Vec<OffersProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#offers)
				})?;
			} else {
				serialize_struct.skip_field("offers")?;
			}
			#[cfg(any(
				any(
					feature = "pattern-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#pattern) {
				serialize_struct.serialize_field("pattern", {
					struct SerializeWith<'a>(&'a Vec<PatternProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#pattern)
				})?;
			} else {
				serialize_struct.skip_field("pattern")?;
			}
			#[cfg(any(
				any(feature = "payload-property-schema", feature = "auto-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#payload) {
				serialize_struct.serialize_field("payload", {
					struct SerializeWith<'a>(&'a Vec<PayloadProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#payload)
				})?;
			} else {
				serialize_struct.skip_field("payload")?;
			}
			#[cfg(any(
				any(
					feature = "positive-notes-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#positive_notes) {
				serialize_struct.serialize_field("positiveNotes", {
					struct SerializeWith<'a>(&'a Vec<PositiveNotesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#positive_notes)
				})?;
			} else {
				serialize_struct.skip_field("positiveNotes")?;
			}
			#[cfg(any(
				any(
					feature = "potential-action-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#potential_action) {
				serialize_struct.serialize_field("potentialAction", {
					struct SerializeWith<'a>(&'a Vec<PotentialActionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#potential_action)
				})?;
			} else {
				serialize_struct.skip_field("potentialAction")?;
			}
			#[cfg(any(
				any(
					feature = "product-id-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#product_id) {
				serialize_struct.serialize_field("productID", {
					struct SerializeWith<'a>(&'a Vec<ProductIdProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#product_id)
				})?;
			} else {
				serialize_struct.skip_field("productID")?;
			}
			#[cfg(any(
				any(
					feature = "production-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#production_date) {
				serialize_struct.serialize_field("productionDate", {
					struct SerializeWith<'a>(&'a Vec<ProductionDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#production_date)
				})?;
			} else {
				serialize_struct.skip_field("productionDate")?;
			}
			#[cfg(any(
				any(
					feature = "purchase-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#purchase_date) {
				serialize_struct.serialize_field("purchaseDate", {
					struct SerializeWith<'a>(&'a Vec<PurchaseDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#purchase_date)
				})?;
			} else {
				serialize_struct.skip_field("purchaseDate")?;
			}
			#[cfg(any(
				any(
					feature = "release-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#release_date) {
				serialize_struct.serialize_field("releaseDate", {
					struct SerializeWith<'a>(&'a Vec<ReleaseDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#release_date)
				})?;
			} else {
				serialize_struct.skip_field("releaseDate")?;
			}
			#[cfg(any(
				any(feature = "review-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#review) {
				serialize_struct.serialize_field("review", {
					struct SerializeWith<'a>(&'a Vec<ReviewProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#review)
				})?;
			} else {
				serialize_struct.skip_field("review")?;
			}
			#[cfg(any(
				any(
					feature = "reviews-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#reviews) {
				serialize_struct.serialize_field("reviews", {
					struct SerializeWith<'a>(&'a Vec<ReviewsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#reviews)
				})?;
			} else {
				serialize_struct.skip_field("reviews")?;
			}
			#[cfg(any(
				any(
					feature = "same-as-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#same_as) {
				serialize_struct.serialize_field("sameAs", {
					struct SerializeWith<'a>(&'a Vec<SameAsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#same_as)
				})?;
			} else {
				serialize_struct.skip_field("sameAs")?;
			}
			#[cfg(any(
				any(
					feature = "seating-capacity-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#seating_capacity) {
				serialize_struct.serialize_field("seatingCapacity", {
					struct SerializeWith<'a>(&'a Vec<SeatingCapacityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#seating_capacity)
				})?;
			} else {
				serialize_struct.skip_field("seatingCapacity")?;
			}
			#[cfg(any(
				any(feature = "size-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#size) {
				serialize_struct.serialize_field("size", {
					struct SerializeWith<'a>(&'a Vec<SizeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#size)
				})?;
			} else {
				serialize_struct.skip_field("size")?;
			}
			#[cfg(any(
				any(feature = "sku-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#sku) {
				serialize_struct.serialize_field("sku", {
					struct SerializeWith<'a>(&'a Vec<SkuProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sku)
				})?;
			} else {
				serialize_struct.skip_field("sku")?;
			}
			#[cfg(any(
				any(feature = "slogan-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#slogan) {
				serialize_struct.serialize_field("slogan", {
					struct SerializeWith<'a>(&'a Vec<SloganProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#slogan)
				})?;
			} else {
				serialize_struct.skip_field("slogan")?;
			}
			#[cfg(any(
				any(feature = "speed-property-schema", feature = "auto-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#speed) {
				serialize_struct.serialize_field("speed", {
					struct SerializeWith<'a>(&'a Vec<SpeedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#speed)
				})?;
			} else {
				serialize_struct.skip_field("speed")?;
			}
			#[cfg(any(
				any(
					feature = "steering-position-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#steering_position) {
				serialize_struct.serialize_field("steeringPosition", {
					struct SerializeWith<'a>(&'a Vec<SteeringPositionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#steering_position)
				})?;
			} else {
				serialize_struct.skip_field("steeringPosition")?;
			}
			#[cfg(any(
				any(
					feature = "stupid-property-property-schema",
					feature = "attic-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#stupid_property) {
				serialize_struct.serialize_field("stupidProperty", {
					struct SerializeWith<'a>(&'a Vec<StupidPropertyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#stupid_property)
				})?;
			} else {
				serialize_struct.skip_field("stupidProperty")?;
			}
			#[cfg(any(
				any(
					feature = "subject-of-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#subject_of) {
				serialize_struct.serialize_field("subjectOf", {
					struct SerializeWith<'a>(&'a Vec<SubjectOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#subject_of)
				})?;
			} else {
				serialize_struct.skip_field("subjectOf")?;
			}
			#[cfg(any(
				any(
					feature = "tongue-weight-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#tongue_weight) {
				serialize_struct.serialize_field("tongueWeight", {
					struct SerializeWith<'a>(&'a Vec<TongueWeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#tongue_weight)
				})?;
			} else {
				serialize_struct.skip_field("tongueWeight")?;
			}
			#[cfg(any(
				any(
					feature = "trailer-weight-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#trailer_weight) {
				serialize_struct.serialize_field("trailerWeight", {
					struct SerializeWith<'a>(&'a Vec<TrailerWeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#trailer_weight)
				})?;
			} else {
				serialize_struct.skip_field("trailerWeight")?;
			}
			#[cfg(any(
				any(feature = "url-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#url) {
				serialize_struct.serialize_field("url", {
					struct SerializeWith<'a>(&'a Vec<UrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#url)
				})?;
			} else {
				serialize_struct.skip_field("url")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-configuration-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_configuration) {
				serialize_struct.serialize_field("vehicleConfiguration", {
					struct SerializeWith<'a>(&'a Vec<VehicleConfigurationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_configuration)
				})?;
			} else {
				serialize_struct.skip_field("vehicleConfiguration")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-engine-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_engine) {
				serialize_struct.serialize_field("vehicleEngine", {
					struct SerializeWith<'a>(&'a Vec<VehicleEngineProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_engine)
				})?;
			} else {
				serialize_struct.skip_field("vehicleEngine")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-identification-number-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_identification_number) {
				serialize_struct.serialize_field("vehicleIdentificationNumber", {
					struct SerializeWith<'a>(&'a Vec<VehicleIdentificationNumberProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_identification_number)
				})?;
			} else {
				serialize_struct.skip_field("vehicleIdentificationNumber")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-interior-color-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_interior_color) {
				serialize_struct.serialize_field("vehicleInteriorColor", {
					struct SerializeWith<'a>(&'a Vec<VehicleInteriorColorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_interior_color)
				})?;
			} else {
				serialize_struct.skip_field("vehicleInteriorColor")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-interior-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_interior_type) {
				serialize_struct.serialize_field("vehicleInteriorType", {
					struct SerializeWith<'a>(&'a Vec<VehicleInteriorTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_interior_type)
				})?;
			} else {
				serialize_struct.skip_field("vehicleInteriorType")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-model-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_model_date) {
				serialize_struct.serialize_field("vehicleModelDate", {
					struct SerializeWith<'a>(&'a Vec<VehicleModelDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_model_date)
				})?;
			} else {
				serialize_struct.skip_field("vehicleModelDate")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-seating-capacity-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_seating_capacity) {
				serialize_struct.serialize_field("vehicleSeatingCapacity", {
					struct SerializeWith<'a>(&'a Vec<VehicleSeatingCapacityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_seating_capacity)
				})?;
			} else {
				serialize_struct.skip_field("vehicleSeatingCapacity")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-special-usage-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_special_usage) {
				serialize_struct.serialize_field("vehicleSpecialUsage", {
					struct SerializeWith<'a>(&'a Vec<VehicleSpecialUsageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_special_usage)
				})?;
			} else {
				serialize_struct.skip_field("vehicleSpecialUsage")?;
			}
			#[cfg(any(
				any(
					feature = "vehicle-transmission-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#vehicle_transmission) {
				serialize_struct.serialize_field("vehicleTransmission", {
					struct SerializeWith<'a>(&'a Vec<VehicleTransmissionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#vehicle_transmission)
				})?;
			} else {
				serialize_struct.skip_field("vehicleTransmission")?;
			}
			#[cfg(any(
				any(feature = "weight-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#weight) {
				serialize_struct.serialize_field("weight", {
					struct SerializeWith<'a>(&'a Vec<WeightProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#weight)
				})?;
			} else {
				serialize_struct.skip_field("weight")?;
			}
			#[cfg(any(
				any(
					feature = "weight-total-property-schema",
					feature = "auto-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#weight_total) {
				serialize_struct.serialize_field("weightTotal", {
					struct SerializeWith<'a>(&'a Vec<WeightTotalProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#weight_total)
				})?;
			} else {
				serialize_struct.skip_field("weightTotal")?;
			}
			#[cfg(any(
				any(feature = "wheelbase-property-schema", feature = "auto-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#wheelbase) {
				serialize_struct.serialize_field("wheelbase", {
					struct SerializeWith<'a>(&'a Vec<WheelbaseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#wheelbase)
				})?;
			} else {
				serialize_struct.skip_field("wheelbase")?;
			}
			#[cfg(any(
				any(feature = "width-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#width) {
				serialize_struct.serialize_field("width", {
					struct SerializeWith<'a>(&'a Vec<WidthProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#width)
				})?;
			} else {
				serialize_struct.skip_field("width")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Vehicle {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "acceleration-time-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				AccelerationTime,
				#[cfg(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdditionalProperty,
				#[cfg(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdditionalType,
				#[cfg(any(
					any(
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AggregateRating,
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternateName,
				#[cfg(any(
					any(feature = "asin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Asin,
				#[cfg(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Audience,
				#[cfg(any(
					any(feature = "award-property-schema", feature = "general-schema-section"),
					doc
				))]
				Award,
				#[cfg(any(
					any(feature = "awards-property-schema", feature = "general-schema-section"),
					doc
				))]
				Awards,
				#[cfg(any(
					any(feature = "body-type-property-schema", feature = "auto-schema-section"),
					doc
				))]
				BodyType,
				#[cfg(any(
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				Brand,
				#[cfg(any(
					any(
						feature = "call-sign-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CallSign,
				#[cfg(any(
					any(
						feature = "cargo-volume-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				CargoVolume,
				#[cfg(any(
					any(
						feature = "category-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Category,
				#[cfg(any(
					any(feature = "color-property-schema", feature = "general-schema-section"),
					doc
				))]
				Color,
				#[cfg(any(
					any(
						feature = "country-of-assembly-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CountryOfAssembly,
				#[cfg(any(
					any(
						feature = "country-of-last-processing-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CountryOfLastProcessing,
				#[cfg(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				CountryOfOrigin,
				#[cfg(any(
					any(
						feature = "date-vehicle-first-registered-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DateVehicleFirstRegistered,
				#[cfg(any(
					any(feature = "depth-property-schema", feature = "general-schema-section"),
					doc
				))]
				Depth,
				#[cfg(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Description,
				#[cfg(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DisambiguatingDescription,
				#[cfg(any(
					any(
						feature = "drive-wheel-configuration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DriveWheelConfiguration,
				#[cfg(any(
					any(
						feature = "emissions-co-2-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				EmissionsCo2,
				#[cfg(any(
					any(
						feature = "fuel-capacity-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				FuelCapacity,
				#[cfg(any(
					any(
						feature = "fuel-consumption-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FuelConsumption,
				#[cfg(any(
					any(
						feature = "fuel-efficiency-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FuelEfficiency,
				#[cfg(any(
					any(
						feature = "fuel-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FuelType,
				#[cfg(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Funding,
				#[cfg(any(
					any(feature = "gtin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Gtin,
				#[cfg(any(
					any(
						feature = "gtin-12-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Gtin12,
				#[cfg(any(
					any(
						feature = "gtin-13-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Gtin13,
				#[cfg(any(
					any(
						feature = "gtin-14-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Gtin14,
				#[cfg(any(
					any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
					doc
				))]
				Gtin8,
				#[cfg(any(
					any(
						feature = "has-adult-consideration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasAdultConsideration,
				#[cfg(any(
					any(
						feature = "has-energy-consumption-details-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasEnergyConsumptionDetails,
				#[cfg(any(
					any(
						feature = "has-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasMeasurement,
				#[cfg(any(
					any(
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasMerchantReturnPolicy,
				#[cfg(any(
					any(
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				))]
				HasProductReturnPolicy,
				#[cfg(any(
					any(feature = "height-property-schema", feature = "general-schema-section"),
					doc
				))]
				Height,
				#[cfg(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Identifier,
				#[cfg(any(
					any(feature = "image-property-schema", feature = "general-schema-section"),
					doc
				))]
				Image,
				#[cfg(any(
					any(
						feature = "in-product-group-with-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				InProductGroupWithId,
				#[cfg(any(
					any(
						feature = "is-accessory-or-spare-part-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsAccessoryOrSparePartFor,
				#[cfg(any(
					any(
						feature = "is-consumable-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsConsumableFor,
				#[cfg(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsFamilyFriendly,
				#[cfg(any(
					any(
						feature = "is-related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsRelatedTo,
				#[cfg(any(
					any(
						feature = "is-similar-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsSimilarTo,
				#[cfg(any(
					any(
						feature = "is-variant-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsVariantOf,
				#[cfg(any(
					any(
						feature = "item-condition-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ItemCondition,
				#[cfg(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Keywords,
				#[cfg(any(
					any(
						feature = "known-vehicle-damages-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				KnownVehicleDamages,
				#[cfg(any(
					any(feature = "logo-property-schema", feature = "general-schema-section"),
					doc
				))]
				Logo,
				#[cfg(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MainEntityOfPage,
				#[cfg(any(
					any(
						feature = "manufacturer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Manufacturer,
				#[cfg(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Material,
				#[cfg(any(
					any(
						feature = "meets-emission-standard-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				MeetsEmissionStandard,
				#[cfg(any(
					any(
						feature = "mileage-from-odometer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MileageFromOdometer,
				#[cfg(any(
					any(
						feature = "mobile-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MobileUrl,
				#[cfg(any(
					any(feature = "model-property-schema", feature = "general-schema-section"),
					doc
				))]
				Model,
				#[cfg(any(
					any(
						feature = "model-date-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				ModelDate,
				#[cfg(any(
					any(feature = "mpn-property-schema", feature = "general-schema-section"),
					doc
				))]
				Mpn,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(
						feature = "negative-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NegativeNotes,
				#[cfg(any(
					any(feature = "nsn-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Nsn,
				#[cfg(any(
					any(
						feature = "number-of-airbags-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NumberOfAirbags,
				#[cfg(any(
					any(
						feature = "number-of-axles-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NumberOfAxles,
				#[cfg(any(
					any(
						feature = "number-of-doors-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NumberOfDoors,
				#[cfg(any(
					any(
						feature = "number-of-forward-gears-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NumberOfForwardGears,
				#[cfg(any(
					any(
						feature = "number-of-previous-owners-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NumberOfPreviousOwners,
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				Offers,
				#[cfg(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Pattern,
				#[cfg(any(
					any(feature = "payload-property-schema", feature = "auto-schema-section"),
					doc
				))]
				Payload,
				#[cfg(any(
					any(
						feature = "positive-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				PositiveNotes,
				#[cfg(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PotentialAction,
				#[cfg(any(
					any(
						feature = "product-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ProductId,
				#[cfg(any(
					any(
						feature = "production-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ProductionDate,
				#[cfg(any(
					any(
						feature = "purchase-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PurchaseDate,
				#[cfg(any(
					any(
						feature = "release-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ReleaseDate,
				#[cfg(any(
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				Review,
				#[cfg(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Reviews,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(
						feature = "seating-capacity-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				SeatingCapacity,
				#[cfg(any(
					any(feature = "size-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Size,
				#[cfg(any(
					any(feature = "sku-property-schema", feature = "general-schema-section"),
					doc
				))]
				Sku,
				#[cfg(any(
					any(feature = "slogan-property-schema", feature = "general-schema-section"),
					doc
				))]
				Slogan,
				#[cfg(any(
					any(feature = "speed-property-schema", feature = "auto-schema-section"),
					doc
				))]
				Speed,
				#[cfg(any(
					any(
						feature = "steering-position-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SteeringPosition,
				#[cfg(any(
					any(
						feature = "stupid-property-property-schema",
						feature = "attic-schema-section"
					),
					doc
				))]
				StupidProperty,
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubjectOf,
				#[cfg(any(
					any(
						feature = "tongue-weight-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				TongueWeight,
				#[cfg(any(
					any(
						feature = "trailer-weight-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				TrailerWeight,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
				#[cfg(any(
					any(
						feature = "vehicle-configuration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VehicleConfiguration,
				#[cfg(any(
					any(
						feature = "vehicle-engine-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VehicleEngine,
				#[cfg(any(
					any(
						feature = "vehicle-identification-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VehicleIdentificationNumber,
				#[cfg(any(
					any(
						feature = "vehicle-interior-color-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VehicleInteriorColor,
				#[cfg(any(
					any(
						feature = "vehicle-interior-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VehicleInteriorType,
				#[cfg(any(
					any(
						feature = "vehicle-model-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VehicleModelDate,
				#[cfg(any(
					any(
						feature = "vehicle-seating-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VehicleSeatingCapacity,
				#[cfg(any(
					any(
						feature = "vehicle-special-usage-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				VehicleSpecialUsage,
				#[cfg(any(
					any(
						feature = "vehicle-transmission-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VehicleTransmission,
				#[cfg(any(
					any(feature = "weight-property-schema", feature = "general-schema-section"),
					doc
				))]
				Weight,
				#[cfg(any(
					any(
						feature = "weight-total-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				WeightTotal,
				#[cfg(any(
					any(feature = "wheelbase-property-schema", feature = "auto-schema-section"),
					doc
				))]
				Wheelbase,
				#[cfg(any(
					any(feature = "width-property-schema", feature = "general-schema-section"),
					doc
				))]
				Width,
				Ignore,
			}
			struct FieldVisitor;
			impl<'de> Visitor<'de> for FieldVisitor {
				type Value = Field;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("field identifier")
				}
				fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						#[cfg(any(
							any(
								feature = "acceleration-time-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"accelerationTime" => Ok(Field::AccelerationTime),
						#[cfg(any(
							any(
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalProperty" => Ok(Field::AdditionalProperty),
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalType" => Ok(Field::AdditionalType),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"aggregateRating" => Ok(Field::AggregateRating),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"asin" => Ok(Field::Asin),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"award" => Ok(Field::Award),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"awards" => Ok(Field::Awards),
						#[cfg(any(
							any(
								feature = "body-type-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"bodyType" => Ok(Field::BodyType),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"brand" => Ok(Field::Brand),
						#[cfg(any(
							any(
								feature = "call-sign-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"callSign" => Ok(Field::CallSign),
						#[cfg(any(
							any(
								feature = "cargo-volume-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"cargoVolume" => Ok(Field::CargoVolume),
						#[cfg(any(
							any(
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"category" => Ok(Field::Category),
						#[cfg(any(
							any(
								feature = "color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"color" => Ok(Field::Color),
						#[cfg(any(
							any(
								feature = "country-of-assembly-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"countryOfAssembly" => Ok(Field::CountryOfAssembly),
						#[cfg(any(
							any(
								feature = "country-of-last-processing-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"countryOfLastProcessing" => Ok(Field::CountryOfLastProcessing),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						#[cfg(any(
							any(
								feature = "date-vehicle-first-registered-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"dateVehicleFirstRegistered" => Ok(Field::DateVehicleFirstRegistered),
						#[cfg(any(
							any(
								feature = "depth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"depth" => Ok(Field::Depth),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"description" => Ok(Field::Description),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						#[cfg(any(
							any(
								feature = "drive-wheel-configuration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"driveWheelConfiguration" => Ok(Field::DriveWheelConfiguration),
						#[cfg(any(
							any(
								feature = "emissions-co-2-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"emissionsCO2" => Ok(Field::EmissionsCo2),
						#[cfg(any(
							any(
								feature = "fuel-capacity-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"fuelCapacity" => Ok(Field::FuelCapacity),
						#[cfg(any(
							any(
								feature = "fuel-consumption-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"fuelConsumption" => Ok(Field::FuelConsumption),
						#[cfg(any(
							any(
								feature = "fuel-efficiency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"fuelEfficiency" => Ok(Field::FuelEfficiency),
						#[cfg(any(
							any(
								feature = "fuel-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"fuelType" => Ok(Field::FuelType),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"funding" => Ok(Field::Funding),
						#[cfg(any(
							any(
								feature = "gtin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"gtin" => Ok(Field::Gtin),
						#[cfg(any(
							any(
								feature = "gtin-12-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"gtin12" => Ok(Field::Gtin12),
						#[cfg(any(
							any(
								feature = "gtin-13-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"gtin13" => Ok(Field::Gtin13),
						#[cfg(any(
							any(
								feature = "gtin-14-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"gtin14" => Ok(Field::Gtin14),
						#[cfg(any(
							any(
								feature = "gtin-8-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"gtin8" => Ok(Field::Gtin8),
						#[cfg(any(
							any(
								feature = "has-adult-consideration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasAdultConsideration" => Ok(Field::HasAdultConsideration),
						#[cfg(any(
							any(
								feature = "has-energy-consumption-details-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasEnergyConsumptionDetails" => Ok(Field::HasEnergyConsumptionDetails),
						#[cfg(any(
							any(
								feature = "has-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasMeasurement" => Ok(Field::HasMeasurement),
						#[cfg(any(
							any(
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						#[cfg(any(
							any(
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"height" => Ok(Field::Height),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"identifier" => Ok(Field::Identifier),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"image" => Ok(Field::Image),
						#[cfg(any(
							any(
								feature = "in-product-group-with-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"inProductGroupWithID" => Ok(Field::InProductGroupWithId),
						#[cfg(any(
							any(
								feature = "is-accessory-or-spare-part-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isAccessoryOrSparePartFor" => Ok(Field::IsAccessoryOrSparePartFor),
						#[cfg(any(
							any(
								feature = "is-consumable-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isConsumableFor" => Ok(Field::IsConsumableFor),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						#[cfg(any(
							any(
								feature = "is-related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isRelatedTo" => Ok(Field::IsRelatedTo),
						#[cfg(any(
							any(
								feature = "is-similar-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isSimilarTo" => Ok(Field::IsSimilarTo),
						#[cfg(any(
							any(
								feature = "is-variant-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isVariantOf" => Ok(Field::IsVariantOf),
						#[cfg(any(
							any(
								feature = "item-condition-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"itemCondition" => Ok(Field::ItemCondition),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"keywords" => Ok(Field::Keywords),
						#[cfg(any(
							any(
								feature = "known-vehicle-damages-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"knownVehicleDamages" => Ok(Field::KnownVehicleDamages),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"logo" => Ok(Field::Logo),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "manufacturer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"manufacturer" => Ok(Field::Manufacturer),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"material" => Ok(Field::Material),
						#[cfg(any(
							any(
								feature = "meets-emission-standard-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"meetsEmissionStandard" => Ok(Field::MeetsEmissionStandard),
						#[cfg(any(
							any(
								feature = "mileage-from-odometer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mileageFromOdometer" => Ok(Field::MileageFromOdometer),
						#[cfg(any(
							any(
								feature = "mobile-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"mobileUrl" => Ok(Field::MobileUrl),
						#[cfg(any(
							any(
								feature = "model-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"model" => Ok(Field::Model),
						#[cfg(any(
							any(
								feature = "model-date-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"modelDate" => Ok(Field::ModelDate),
						#[cfg(any(
							any(
								feature = "mpn-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mpn" => Ok(Field::Mpn),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"name" => Ok(Field::Name),
						#[cfg(any(
							any(
								feature = "negative-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"negativeNotes" => Ok(Field::NegativeNotes),
						#[cfg(any(
							any(
								feature = "nsn-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"nsn" => Ok(Field::Nsn),
						#[cfg(any(
							any(
								feature = "number-of-airbags-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"numberOfAirbags" => Ok(Field::NumberOfAirbags),
						#[cfg(any(
							any(
								feature = "number-of-axles-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"numberOfAxles" => Ok(Field::NumberOfAxles),
						#[cfg(any(
							any(
								feature = "number-of-doors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"numberOfDoors" => Ok(Field::NumberOfDoors),
						#[cfg(any(
							any(
								feature = "number-of-forward-gears-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"numberOfForwardGears" => Ok(Field::NumberOfForwardGears),
						#[cfg(any(
							any(
								feature = "number-of-previous-owners-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"numberOfPreviousOwners" => Ok(Field::NumberOfPreviousOwners),
						#[cfg(any(
							any(
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"offers" => Ok(Field::Offers),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"pattern" => Ok(Field::Pattern),
						#[cfg(any(
							any(
								feature = "payload-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"payload" => Ok(Field::Payload),
						#[cfg(any(
							any(
								feature = "positive-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"positiveNotes" => Ok(Field::PositiveNotes),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "product-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"productID" => Ok(Field::ProductId),
						#[cfg(any(
							any(
								feature = "production-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"productionDate" => Ok(Field::ProductionDate),
						#[cfg(any(
							any(
								feature = "purchase-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"purchaseDate" => Ok(Field::PurchaseDate),
						#[cfg(any(
							any(
								feature = "release-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"releaseDate" => Ok(Field::ReleaseDate),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"review" => Ok(Field::Review),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"reviews" => Ok(Field::Reviews),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "seating-capacity-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"seatingCapacity" => Ok(Field::SeatingCapacity),
						#[cfg(any(
							any(
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"size" => Ok(Field::Size),
						#[cfg(any(
							any(
								feature = "sku-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sku" => Ok(Field::Sku),
						#[cfg(any(
							any(
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"slogan" => Ok(Field::Slogan),
						#[cfg(any(
							any(
								feature = "speed-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"speed" => Ok(Field::Speed),
						#[cfg(any(
							any(
								feature = "steering-position-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"steeringPosition" => Ok(Field::SteeringPosition),
						#[cfg(any(
							any(
								feature = "stupid-property-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						"stupidProperty" => Ok(Field::StupidProperty),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"subjectOf" => Ok(Field::SubjectOf),
						#[cfg(any(
							any(
								feature = "tongue-weight-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"tongueWeight" => Ok(Field::TongueWeight),
						#[cfg(any(
							any(
								feature = "trailer-weight-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"trailerWeight" => Ok(Field::TrailerWeight),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"url" => Ok(Field::Url),
						#[cfg(any(
							any(
								feature = "vehicle-configuration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vehicleConfiguration" => Ok(Field::VehicleConfiguration),
						#[cfg(any(
							any(
								feature = "vehicle-engine-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vehicleEngine" => Ok(Field::VehicleEngine),
						#[cfg(any(
							any(
								feature = "vehicle-identification-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vehicleIdentificationNumber" => Ok(Field::VehicleIdentificationNumber),
						#[cfg(any(
							any(
								feature = "vehicle-interior-color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vehicleInteriorColor" => Ok(Field::VehicleInteriorColor),
						#[cfg(any(
							any(
								feature = "vehicle-interior-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vehicleInteriorType" => Ok(Field::VehicleInteriorType),
						#[cfg(any(
							any(
								feature = "vehicle-model-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vehicleModelDate" => Ok(Field::VehicleModelDate),
						#[cfg(any(
							any(
								feature = "vehicle-seating-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vehicleSeatingCapacity" => Ok(Field::VehicleSeatingCapacity),
						#[cfg(any(
							any(
								feature = "vehicle-special-usage-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"vehicleSpecialUsage" => Ok(Field::VehicleSpecialUsage),
						#[cfg(any(
							any(
								feature = "vehicle-transmission-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vehicleTransmission" => Ok(Field::VehicleTransmission),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"weight" => Ok(Field::Weight),
						#[cfg(any(
							any(
								feature = "weight-total-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"weightTotal" => Ok(Field::WeightTotal),
						#[cfg(any(
							any(
								feature = "wheelbase-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						"wheelbase" => Ok(Field::Wheelbase),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"width" => Ok(Field::Width),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						#[cfg(any(
							any(
								feature = "acceleration-time-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"accelerationTime" => Ok(Field::AccelerationTime),
						#[cfg(any(
							any(
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalProperty" => Ok(Field::AdditionalProperty),
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalType" => Ok(Field::AdditionalType),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"aggregateRating" => Ok(Field::AggregateRating),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"asin" => Ok(Field::Asin),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"award" => Ok(Field::Award),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"awards" => Ok(Field::Awards),
						#[cfg(any(
							any(
								feature = "body-type-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"bodyType" => Ok(Field::BodyType),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"brand" => Ok(Field::Brand),
						#[cfg(any(
							any(
								feature = "call-sign-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"callSign" => Ok(Field::CallSign),
						#[cfg(any(
							any(
								feature = "cargo-volume-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"cargoVolume" => Ok(Field::CargoVolume),
						#[cfg(any(
							any(
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"category" => Ok(Field::Category),
						#[cfg(any(
							any(
								feature = "color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"color" => Ok(Field::Color),
						#[cfg(any(
							any(
								feature = "country-of-assembly-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"countryOfAssembly" => Ok(Field::CountryOfAssembly),
						#[cfg(any(
							any(
								feature = "country-of-last-processing-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"countryOfLastProcessing" => Ok(Field::CountryOfLastProcessing),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						#[cfg(any(
							any(
								feature = "date-vehicle-first-registered-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"dateVehicleFirstRegistered" => Ok(Field::DateVehicleFirstRegistered),
						#[cfg(any(
							any(
								feature = "depth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"depth" => Ok(Field::Depth),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"description" => Ok(Field::Description),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						#[cfg(any(
							any(
								feature = "drive-wheel-configuration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"driveWheelConfiguration" => Ok(Field::DriveWheelConfiguration),
						#[cfg(any(
							any(
								feature = "emissions-co-2-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"emissionsCO2" => Ok(Field::EmissionsCo2),
						#[cfg(any(
							any(
								feature = "fuel-capacity-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"fuelCapacity" => Ok(Field::FuelCapacity),
						#[cfg(any(
							any(
								feature = "fuel-consumption-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"fuelConsumption" => Ok(Field::FuelConsumption),
						#[cfg(any(
							any(
								feature = "fuel-efficiency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"fuelEfficiency" => Ok(Field::FuelEfficiency),
						#[cfg(any(
							any(
								feature = "fuel-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"fuelType" => Ok(Field::FuelType),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"funding" => Ok(Field::Funding),
						#[cfg(any(
							any(
								feature = "gtin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"gtin" => Ok(Field::Gtin),
						#[cfg(any(
							any(
								feature = "gtin-12-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"gtin12" => Ok(Field::Gtin12),
						#[cfg(any(
							any(
								feature = "gtin-13-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"gtin13" => Ok(Field::Gtin13),
						#[cfg(any(
							any(
								feature = "gtin-14-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"gtin14" => Ok(Field::Gtin14),
						#[cfg(any(
							any(
								feature = "gtin-8-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"gtin8" => Ok(Field::Gtin8),
						#[cfg(any(
							any(
								feature = "has-adult-consideration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasAdultConsideration" => Ok(Field::HasAdultConsideration),
						#[cfg(any(
							any(
								feature = "has-energy-consumption-details-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasEnergyConsumptionDetails" => Ok(Field::HasEnergyConsumptionDetails),
						#[cfg(any(
							any(
								feature = "has-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasMeasurement" => Ok(Field::HasMeasurement),
						#[cfg(any(
							any(
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						#[cfg(any(
							any(
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						b"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"height" => Ok(Field::Height),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"identifier" => Ok(Field::Identifier),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"image" => Ok(Field::Image),
						#[cfg(any(
							any(
								feature = "in-product-group-with-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"inProductGroupWithID" => Ok(Field::InProductGroupWithId),
						#[cfg(any(
							any(
								feature = "is-accessory-or-spare-part-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isAccessoryOrSparePartFor" => Ok(Field::IsAccessoryOrSparePartFor),
						#[cfg(any(
							any(
								feature = "is-consumable-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isConsumableFor" => Ok(Field::IsConsumableFor),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						#[cfg(any(
							any(
								feature = "is-related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isRelatedTo" => Ok(Field::IsRelatedTo),
						#[cfg(any(
							any(
								feature = "is-similar-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isSimilarTo" => Ok(Field::IsSimilarTo),
						#[cfg(any(
							any(
								feature = "is-variant-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isVariantOf" => Ok(Field::IsVariantOf),
						#[cfg(any(
							any(
								feature = "item-condition-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"itemCondition" => Ok(Field::ItemCondition),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"keywords" => Ok(Field::Keywords),
						#[cfg(any(
							any(
								feature = "known-vehicle-damages-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"knownVehicleDamages" => Ok(Field::KnownVehicleDamages),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"logo" => Ok(Field::Logo),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "manufacturer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"manufacturer" => Ok(Field::Manufacturer),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"material" => Ok(Field::Material),
						#[cfg(any(
							any(
								feature = "meets-emission-standard-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"meetsEmissionStandard" => Ok(Field::MeetsEmissionStandard),
						#[cfg(any(
							any(
								feature = "mileage-from-odometer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mileageFromOdometer" => Ok(Field::MileageFromOdometer),
						#[cfg(any(
							any(
								feature = "mobile-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"mobileUrl" => Ok(Field::MobileUrl),
						#[cfg(any(
							any(
								feature = "model-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"model" => Ok(Field::Model),
						#[cfg(any(
							any(
								feature = "model-date-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"modelDate" => Ok(Field::ModelDate),
						#[cfg(any(
							any(
								feature = "mpn-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mpn" => Ok(Field::Mpn),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"name" => Ok(Field::Name),
						#[cfg(any(
							any(
								feature = "negative-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"negativeNotes" => Ok(Field::NegativeNotes),
						#[cfg(any(
							any(
								feature = "nsn-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"nsn" => Ok(Field::Nsn),
						#[cfg(any(
							any(
								feature = "number-of-airbags-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"numberOfAirbags" => Ok(Field::NumberOfAirbags),
						#[cfg(any(
							any(
								feature = "number-of-axles-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"numberOfAxles" => Ok(Field::NumberOfAxles),
						#[cfg(any(
							any(
								feature = "number-of-doors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"numberOfDoors" => Ok(Field::NumberOfDoors),
						#[cfg(any(
							any(
								feature = "number-of-forward-gears-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"numberOfForwardGears" => Ok(Field::NumberOfForwardGears),
						#[cfg(any(
							any(
								feature = "number-of-previous-owners-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"numberOfPreviousOwners" => Ok(Field::NumberOfPreviousOwners),
						#[cfg(any(
							any(
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"offers" => Ok(Field::Offers),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"pattern" => Ok(Field::Pattern),
						#[cfg(any(
							any(
								feature = "payload-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"payload" => Ok(Field::Payload),
						#[cfg(any(
							any(
								feature = "positive-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"positiveNotes" => Ok(Field::PositiveNotes),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "product-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"productID" => Ok(Field::ProductId),
						#[cfg(any(
							any(
								feature = "production-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"productionDate" => Ok(Field::ProductionDate),
						#[cfg(any(
							any(
								feature = "purchase-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"purchaseDate" => Ok(Field::PurchaseDate),
						#[cfg(any(
							any(
								feature = "release-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"releaseDate" => Ok(Field::ReleaseDate),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"review" => Ok(Field::Review),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"reviews" => Ok(Field::Reviews),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "seating-capacity-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"seatingCapacity" => Ok(Field::SeatingCapacity),
						#[cfg(any(
							any(
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"size" => Ok(Field::Size),
						#[cfg(any(
							any(
								feature = "sku-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sku" => Ok(Field::Sku),
						#[cfg(any(
							any(
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"slogan" => Ok(Field::Slogan),
						#[cfg(any(
							any(
								feature = "speed-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"speed" => Ok(Field::Speed),
						#[cfg(any(
							any(
								feature = "steering-position-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"steeringPosition" => Ok(Field::SteeringPosition),
						#[cfg(any(
							any(
								feature = "stupid-property-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						b"stupidProperty" => Ok(Field::StupidProperty),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"subjectOf" => Ok(Field::SubjectOf),
						#[cfg(any(
							any(
								feature = "tongue-weight-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"tongueWeight" => Ok(Field::TongueWeight),
						#[cfg(any(
							any(
								feature = "trailer-weight-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"trailerWeight" => Ok(Field::TrailerWeight),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"url" => Ok(Field::Url),
						#[cfg(any(
							any(
								feature = "vehicle-configuration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vehicleConfiguration" => Ok(Field::VehicleConfiguration),
						#[cfg(any(
							any(
								feature = "vehicle-engine-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vehicleEngine" => Ok(Field::VehicleEngine),
						#[cfg(any(
							any(
								feature = "vehicle-identification-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vehicleIdentificationNumber" => Ok(Field::VehicleIdentificationNumber),
						#[cfg(any(
							any(
								feature = "vehicle-interior-color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vehicleInteriorColor" => Ok(Field::VehicleInteriorColor),
						#[cfg(any(
							any(
								feature = "vehicle-interior-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vehicleInteriorType" => Ok(Field::VehicleInteriorType),
						#[cfg(any(
							any(
								feature = "vehicle-model-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vehicleModelDate" => Ok(Field::VehicleModelDate),
						#[cfg(any(
							any(
								feature = "vehicle-seating-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vehicleSeatingCapacity" => Ok(Field::VehicleSeatingCapacity),
						#[cfg(any(
							any(
								feature = "vehicle-special-usage-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"vehicleSpecialUsage" => Ok(Field::VehicleSpecialUsage),
						#[cfg(any(
							any(
								feature = "vehicle-transmission-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vehicleTransmission" => Ok(Field::VehicleTransmission),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"weight" => Ok(Field::Weight),
						#[cfg(any(
							any(
								feature = "weight-total-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"weightTotal" => Ok(Field::WeightTotal),
						#[cfg(any(
							any(
								feature = "wheelbase-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						b"wheelbase" => Ok(Field::Wheelbase),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"width" => Ok(Field::Width),
						_ => Ok(Field::Ignore),
					}
				}
			}
			impl<'de> Deserialize<'de> for Field {
				fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where
					D: Deserializer<'de>,
				{
					deserializer.deserialize_identifier(FieldVisitor)
				}
			}
			struct ClassVisitor;
			impl<'de> Visitor<'de> for ClassVisitor {
				type Value = Vehicle;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Vehicle")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "acceleration-time-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#acceleration_time_property = None;
					#[cfg(any(
						any(
							feature = "additional-property-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#additional_property_property = None;
					#[cfg(any(
						any(
							feature = "additional-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#additional_type_property = None;
					#[cfg(any(
						any(
							feature = "aggregate-rating-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#aggregate_rating_property = None;
					#[cfg(any(
						any(
							feature = "alternate-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alternate_name_property = None;
					#[cfg(any(
						any(feature = "asin-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#asin_property = None;
					#[cfg(any(
						any(
							feature = "audience-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#audience_property = None;
					#[cfg(any(
						any(feature = "award-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#award_property = None;
					#[cfg(any(
						any(
							feature = "awards-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#awards_property = None;
					#[cfg(any(
						any(
							feature = "body-type-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#body_type_property = None;
					#[cfg(any(
						any(feature = "brand-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#brand_property = None;
					#[cfg(any(
						any(
							feature = "call-sign-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#call_sign_property = None;
					#[cfg(any(
						any(
							feature = "cargo-volume-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#cargo_volume_property = None;
					#[cfg(any(
						any(
							feature = "category-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#category_property = None;
					#[cfg(any(
						any(feature = "color-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#color_property = None;
					#[cfg(any(
						any(
							feature = "country-of-assembly-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#country_of_assembly_property = None;
					#[cfg(any(
						any(
							feature = "country-of-last-processing-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#country_of_last_processing_property = None;
					#[cfg(any(
						any(
							feature = "country-of-origin-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#country_of_origin_property = None;
					#[cfg(any(
						any(
							feature = "date-vehicle-first-registered-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#date_vehicle_first_registered_property = None;
					#[cfg(any(
						any(feature = "depth-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#depth_property = None;
					#[cfg(any(
						any(
							feature = "description-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#description_property = None;
					#[cfg(any(
						any(
							feature = "disambiguating-description-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#disambiguating_description_property = None;
					#[cfg(any(
						any(
							feature = "drive-wheel-configuration-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#drive_wheel_configuration_property = None;
					#[cfg(any(
						any(
							feature = "emissions-co-2-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#emissions_co_2_property = None;
					#[cfg(any(
						any(
							feature = "fuel-capacity-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#fuel_capacity_property = None;
					#[cfg(any(
						any(
							feature = "fuel-consumption-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#fuel_consumption_property = None;
					#[cfg(any(
						any(
							feature = "fuel-efficiency-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#fuel_efficiency_property = None;
					#[cfg(any(
						any(
							feature = "fuel-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#fuel_type_property = None;
					#[cfg(any(
						any(
							feature = "funding-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#funding_property = None;
					#[cfg(any(
						any(feature = "gtin-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#gtin_property = None;
					#[cfg(any(
						any(
							feature = "gtin-12-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#gtin_12_property = None;
					#[cfg(any(
						any(
							feature = "gtin-13-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#gtin_13_property = None;
					#[cfg(any(
						any(
							feature = "gtin-14-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#gtin_14_property = None;
					#[cfg(any(
						any(
							feature = "gtin-8-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#gtin_8_property = None;
					#[cfg(any(
						any(
							feature = "has-adult-consideration-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_adult_consideration_property = None;
					#[cfg(any(
						any(
							feature = "has-energy-consumption-details-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_energy_consumption_details_property = None;
					#[cfg(any(
						any(
							feature = "has-measurement-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_measurement_property = None;
					#[cfg(any(
						any(
							feature = "has-merchant-return-policy-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_merchant_return_policy_property = None;
					#[cfg(any(
						any(
							feature = "has-product-return-policy-property-schema",
							feature = "attic-schema-section"
						),
						doc
					))]
					let mut r#has_product_return_policy_property = None;
					#[cfg(any(
						any(
							feature = "height-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#height_property = None;
					#[cfg(any(
						any(
							feature = "identifier-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#identifier_property = None;
					#[cfg(any(
						any(feature = "image-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#image_property = None;
					#[cfg(any(
						any(
							feature = "in-product-group-with-id-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#in_product_group_with_id_property = None;
					#[cfg(any(
						any(
							feature = "is-accessory-or-spare-part-for-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_accessory_or_spare_part_for_property = None;
					#[cfg(any(
						any(
							feature = "is-consumable-for-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_consumable_for_property = None;
					#[cfg(any(
						any(
							feature = "is-family-friendly-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_family_friendly_property = None;
					#[cfg(any(
						any(
							feature = "is-related-to-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_related_to_property = None;
					#[cfg(any(
						any(
							feature = "is-similar-to-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_similar_to_property = None;
					#[cfg(any(
						any(
							feature = "is-variant-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_variant_of_property = None;
					#[cfg(any(
						any(
							feature = "item-condition-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#item_condition_property = None;
					#[cfg(any(
						any(
							feature = "keywords-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#keywords_property = None;
					#[cfg(any(
						any(
							feature = "known-vehicle-damages-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#known_vehicle_damages_property = None;
					#[cfg(any(
						any(feature = "logo-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#logo_property = None;
					#[cfg(any(
						any(
							feature = "main-entity-of-page-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#main_entity_of_page_property = None;
					#[cfg(any(
						any(
							feature = "manufacturer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#manufacturer_property = None;
					#[cfg(any(
						any(
							feature = "material-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#material_property = None;
					#[cfg(any(
						any(
							feature = "meets-emission-standard-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#meets_emission_standard_property = None;
					#[cfg(any(
						any(
							feature = "mileage-from-odometer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#mileage_from_odometer_property = None;
					#[cfg(any(
						any(
							feature = "mobile-url-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#mobile_url_property = None;
					#[cfg(any(
						any(feature = "model-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#model_property = None;
					#[cfg(any(
						any(
							feature = "model-date-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#model_date_property = None;
					#[cfg(any(
						any(feature = "mpn-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#mpn_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "negative-notes-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#negative_notes_property = None;
					#[cfg(any(
						any(feature = "nsn-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#nsn_property = None;
					#[cfg(any(
						any(
							feature = "number-of-airbags-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#number_of_airbags_property = None;
					#[cfg(any(
						any(
							feature = "number-of-axles-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#number_of_axles_property = None;
					#[cfg(any(
						any(
							feature = "number-of-doors-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#number_of_doors_property = None;
					#[cfg(any(
						any(
							feature = "number-of-forward-gears-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#number_of_forward_gears_property = None;
					#[cfg(any(
						any(
							feature = "number-of-previous-owners-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#number_of_previous_owners_property = None;
					#[cfg(any(
						any(
							feature = "offers-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#offers_property = None;
					#[cfg(any(
						any(
							feature = "pattern-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#pattern_property = None;
					#[cfg(any(
						any(feature = "payload-property-schema", feature = "auto-schema-section"),
						doc
					))]
					let mut r#payload_property = None;
					#[cfg(any(
						any(
							feature = "positive-notes-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#positive_notes_property = None;
					#[cfg(any(
						any(
							feature = "potential-action-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#potential_action_property = None;
					#[cfg(any(
						any(
							feature = "product-id-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#product_id_property = None;
					#[cfg(any(
						any(
							feature = "production-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#production_date_property = None;
					#[cfg(any(
						any(
							feature = "purchase-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#purchase_date_property = None;
					#[cfg(any(
						any(
							feature = "release-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#release_date_property = None;
					#[cfg(any(
						any(
							feature = "review-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#review_property = None;
					#[cfg(any(
						any(
							feature = "reviews-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#reviews_property = None;
					#[cfg(any(
						any(
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(
							feature = "seating-capacity-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#seating_capacity_property = None;
					#[cfg(any(
						any(feature = "size-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#size_property = None;
					#[cfg(any(
						any(feature = "sku-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#sku_property = None;
					#[cfg(any(
						any(
							feature = "slogan-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#slogan_property = None;
					#[cfg(any(
						any(feature = "speed-property-schema", feature = "auto-schema-section"),
						doc
					))]
					let mut r#speed_property = None;
					#[cfg(any(
						any(
							feature = "steering-position-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#steering_position_property = None;
					#[cfg(any(
						any(
							feature = "stupid-property-property-schema",
							feature = "attic-schema-section"
						),
						doc
					))]
					let mut r#stupid_property_property = None;
					#[cfg(any(
						any(
							feature = "subject-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#subject_of_property = None;
					#[cfg(any(
						any(
							feature = "tongue-weight-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#tongue_weight_property = None;
					#[cfg(any(
						any(
							feature = "trailer-weight-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#trailer_weight_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-configuration-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vehicle_configuration_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-engine-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vehicle_engine_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-identification-number-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vehicle_identification_number_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-interior-color-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vehicle_interior_color_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-interior-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vehicle_interior_type_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-model-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vehicle_model_date_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-seating-capacity-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vehicle_seating_capacity_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-special-usage-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#vehicle_special_usage_property = None;
					#[cfg(any(
						any(
							feature = "vehicle-transmission-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vehicle_transmission_property = None;
					#[cfg(any(
						any(
							feature = "weight-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#weight_property = None;
					#[cfg(any(
						any(
							feature = "weight-total-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#weight_total_property = None;
					#[cfg(any(
						any(
							feature = "wheelbase-property-schema",
							feature = "auto-schema-section"
						),
						doc
					))]
					let mut r#wheelbase_property = None;
					#[cfg(any(
						any(feature = "width-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#width_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "acceleration-time-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::AccelerationTime => {
								if r#acceleration_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accelerationTime",
									));
								}
								r#acceleration_time_property = Some({
									struct DeserializeWith(Vec<AccelerationTimeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "additional-property-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AdditionalProperty => {
								if r#additional_property_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalProperty",
									));
								}
								r#additional_property_property = Some({
									struct DeserializeWith(Vec<AdditionalPropertyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "additional-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AdditionalType => {
								if r#additional_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalType",
									));
								}
								r#additional_type_property = Some({
									struct DeserializeWith(Vec<AdditionalTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "aggregate-rating-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AggregateRating => {
								if r#aggregate_rating_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"aggregateRating",
									));
								}
								r#aggregate_rating_property = Some({
									struct DeserializeWith(Vec<AggregateRatingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "alternate-name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AlternateName => {
								if r#alternate_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"alternateName",
									));
								}
								r#alternate_name_property = Some({
									struct DeserializeWith(Vec<AlternateNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "asin-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Asin => {
								if r#asin_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("asin"));
								}
								r#asin_property = Some({
									struct DeserializeWith(Vec<AsinProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "audience-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Audience => {
								if r#audience_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"audience",
									));
								}
								r#audience_property = Some({
									struct DeserializeWith(Vec<AudienceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "award-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Award => {
								if r#award_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("award"));
								}
								r#award_property = Some({
									struct DeserializeWith(Vec<AwardProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "awards-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Awards => {
								if r#awards_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("awards"));
								}
								r#awards_property = Some({
									struct DeserializeWith(Vec<AwardsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "body-type-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::BodyType => {
								if r#body_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"bodyType",
									));
								}
								r#body_type_property = Some({
									struct DeserializeWith(Vec<BodyTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "brand-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Brand => {
								if r#brand_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("brand"));
								}
								r#brand_property = Some({
									struct DeserializeWith(Vec<BrandProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "call-sign-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::CallSign => {
								if r#call_sign_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"callSign",
									));
								}
								r#call_sign_property = Some({
									struct DeserializeWith(Vec<CallSignProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "cargo-volume-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::CargoVolume => {
								if r#cargo_volume_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cargoVolume",
									));
								}
								r#cargo_volume_property = Some({
									struct DeserializeWith(Vec<CargoVolumeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "category-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Category => {
								if r#category_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"category",
									));
								}
								r#category_property = Some({
									struct DeserializeWith(Vec<CategoryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "color-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Color => {
								if r#color_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("color"));
								}
								r#color_property = Some({
									struct DeserializeWith(Vec<ColorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "country-of-assembly-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::CountryOfAssembly => {
								if r#country_of_assembly_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"countryOfAssembly",
									));
								}
								r#country_of_assembly_property = Some({
									struct DeserializeWith(Vec<CountryOfAssemblyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "country-of-last-processing-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::CountryOfLastProcessing => {
								if r#country_of_last_processing_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"countryOfLastProcessing",
									));
								}
								r#country_of_last_processing_property = Some({
									struct DeserializeWith(Vec<CountryOfLastProcessingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "country-of-origin-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::CountryOfOrigin => {
								if r#country_of_origin_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"countryOfOrigin",
									));
								}
								r#country_of_origin_property = Some({
									struct DeserializeWith(Vec<CountryOfOriginProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "date-vehicle-first-registered-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DateVehicleFirstRegistered => {
								if r#date_vehicle_first_registered_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"dateVehicleFirstRegistered",
									));
								}
								r#date_vehicle_first_registered_property = Some({
									struct DeserializeWith(Vec<DateVehicleFirstRegisteredProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "depth-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Depth => {
								if r#depth_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("depth"));
								}
								r#depth_property = Some({
									struct DeserializeWith(Vec<DepthProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "description-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Description => {
								if r#description_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"description",
									));
								}
								r#description_property = Some({
									struct DeserializeWith(Vec<DescriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "disambiguating-description-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DisambiguatingDescription => {
								if r#disambiguating_description_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"disambiguatingDescription",
									));
								}
								r#disambiguating_description_property = Some({
									struct DeserializeWith(Vec<DisambiguatingDescriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "drive-wheel-configuration-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DriveWheelConfiguration => {
								if r#drive_wheel_configuration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"driveWheelConfiguration",
									));
								}
								r#drive_wheel_configuration_property = Some({
									struct DeserializeWith(Vec<DriveWheelConfigurationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "emissions-co-2-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::EmissionsCo2 => {
								if r#emissions_co_2_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"emissionsCO2",
									));
								}
								r#emissions_co_2_property = Some({
									struct DeserializeWith(Vec<EmissionsCo2Property>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "fuel-capacity-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::FuelCapacity => {
								if r#fuel_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fuelCapacity",
									));
								}
								r#fuel_capacity_property = Some({
									struct DeserializeWith(Vec<FuelCapacityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "fuel-consumption-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FuelConsumption => {
								if r#fuel_consumption_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fuelConsumption",
									));
								}
								r#fuel_consumption_property = Some({
									struct DeserializeWith(Vec<FuelConsumptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "fuel-efficiency-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FuelEfficiency => {
								if r#fuel_efficiency_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fuelEfficiency",
									));
								}
								r#fuel_efficiency_property = Some({
									struct DeserializeWith(Vec<FuelEfficiencyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "fuel-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FuelType => {
								if r#fuel_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fuelType",
									));
								}
								r#fuel_type_property = Some({
									struct DeserializeWith(Vec<FuelTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "funding-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Funding => {
								if r#funding_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"funding",
									));
								}
								r#funding_property = Some({
									struct DeserializeWith(Vec<FundingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "gtin-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Gtin => {
								if r#gtin_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin"));
								}
								r#gtin_property = Some({
									struct DeserializeWith(Vec<GtinProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "gtin-12-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Gtin12 => {
								if r#gtin_12_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin12"));
								}
								r#gtin_12_property = Some({
									struct DeserializeWith(Vec<Gtin12Property>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "gtin-13-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Gtin13 => {
								if r#gtin_13_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin13"));
								}
								r#gtin_13_property = Some({
									struct DeserializeWith(Vec<Gtin13Property>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "gtin-14-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Gtin14 => {
								if r#gtin_14_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin14"));
								}
								r#gtin_14_property = Some({
									struct DeserializeWith(Vec<Gtin14Property>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "gtin-8-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Gtin8 => {
								if r#gtin_8_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gtin8"));
								}
								r#gtin_8_property = Some({
									struct DeserializeWith(Vec<Gtin8Property>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "has-adult-consideration-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasAdultConsideration => {
								if r#has_adult_consideration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasAdultConsideration",
									));
								}
								r#has_adult_consideration_property = Some({
									struct DeserializeWith(Vec<HasAdultConsiderationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "has-energy-consumption-details-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasEnergyConsumptionDetails => {
								if r#has_energy_consumption_details_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasEnergyConsumptionDetails",
									));
								}
								r#has_energy_consumption_details_property = Some({
									struct DeserializeWith(
										Vec<HasEnergyConsumptionDetailsProperty>,
									);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "has-measurement-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasMeasurement => {
								if r#has_measurement_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasMeasurement",
									));
								}
								r#has_measurement_property = Some({
									struct DeserializeWith(Vec<HasMeasurementProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "has-merchant-return-policy-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasMerchantReturnPolicy => {
								if r#has_merchant_return_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasMerchantReturnPolicy",
									));
								}
								r#has_merchant_return_policy_property = Some({
									struct DeserializeWith(Vec<HasMerchantReturnPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "has-product-return-policy-property-schema",
									feature = "attic-schema-section"
								),
								doc
							))]
							Field::HasProductReturnPolicy => {
								if r#has_product_return_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasProductReturnPolicy",
									));
								}
								r#has_product_return_policy_property = Some({
									struct DeserializeWith(Vec<HasProductReturnPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "height-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Height => {
								if r#height_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("height"));
								}
								r#height_property = Some({
									struct DeserializeWith(Vec<HeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "identifier-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Identifier => {
								if r#identifier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"identifier",
									));
								}
								r#identifier_property = Some({
									struct DeserializeWith(Vec<IdentifierProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "image-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Image => {
								if r#image_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("image"));
								}
								r#image_property = Some({
									struct DeserializeWith(Vec<ImageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "in-product-group-with-id-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::InProductGroupWithId => {
								if r#in_product_group_with_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"inProductGroupWithID",
									));
								}
								r#in_product_group_with_id_property = Some({
									struct DeserializeWith(Vec<InProductGroupWithIdProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "is-accessory-or-spare-part-for-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IsAccessoryOrSparePartFor => {
								if r#is_accessory_or_spare_part_for_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isAccessoryOrSparePartFor",
									));
								}
								r#is_accessory_or_spare_part_for_property = Some({
									struct DeserializeWith(Vec<IsAccessoryOrSparePartForProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "is-consumable-for-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IsConsumableFor => {
								if r#is_consumable_for_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isConsumableFor",
									));
								}
								r#is_consumable_for_property = Some({
									struct DeserializeWith(Vec<IsConsumableForProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "is-family-friendly-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IsFamilyFriendly => {
								if r#is_family_friendly_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isFamilyFriendly",
									));
								}
								r#is_family_friendly_property = Some({
									struct DeserializeWith(Vec<IsFamilyFriendlyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "is-related-to-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IsRelatedTo => {
								if r#is_related_to_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isRelatedTo",
									));
								}
								r#is_related_to_property = Some({
									struct DeserializeWith(Vec<IsRelatedToProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "is-similar-to-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IsSimilarTo => {
								if r#is_similar_to_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isSimilarTo",
									));
								}
								r#is_similar_to_property = Some({
									struct DeserializeWith(Vec<IsSimilarToProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "is-variant-of-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IsVariantOf => {
								if r#is_variant_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isVariantOf",
									));
								}
								r#is_variant_of_property = Some({
									struct DeserializeWith(Vec<IsVariantOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "item-condition-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ItemCondition => {
								if r#item_condition_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"itemCondition",
									));
								}
								r#item_condition_property = Some({
									struct DeserializeWith(Vec<ItemConditionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "keywords-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Keywords => {
								if r#keywords_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"keywords",
									));
								}
								r#keywords_property = Some({
									struct DeserializeWith(Vec<KeywordsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "known-vehicle-damages-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::KnownVehicleDamages => {
								if r#known_vehicle_damages_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"knownVehicleDamages",
									));
								}
								r#known_vehicle_damages_property = Some({
									struct DeserializeWith(Vec<KnownVehicleDamagesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "logo-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Logo => {
								if r#logo_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("logo"));
								}
								r#logo_property = Some({
									struct DeserializeWith(Vec<LogoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "main-entity-of-page-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MainEntityOfPage => {
								if r#main_entity_of_page_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mainEntityOfPage",
									));
								}
								r#main_entity_of_page_property = Some({
									struct DeserializeWith(Vec<MainEntityOfPageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "manufacturer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Manufacturer => {
								if r#manufacturer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"manufacturer",
									));
								}
								r#manufacturer_property = Some({
									struct DeserializeWith(Vec<ManufacturerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "material-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Material => {
								if r#material_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"material",
									));
								}
								r#material_property = Some({
									struct DeserializeWith(Vec<MaterialProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "meets-emission-standard-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::MeetsEmissionStandard => {
								if r#meets_emission_standard_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"meetsEmissionStandard",
									));
								}
								r#meets_emission_standard_property = Some({
									struct DeserializeWith(Vec<MeetsEmissionStandardProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "mileage-from-odometer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MileageFromOdometer => {
								if r#mileage_from_odometer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mileageFromOdometer",
									));
								}
								r#mileage_from_odometer_property = Some({
									struct DeserializeWith(Vec<MileageFromOdometerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "mobile-url-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MobileUrl => {
								if r#mobile_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mobileUrl",
									));
								}
								r#mobile_url_property = Some({
									struct DeserializeWith(Vec<MobileUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "model-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Model => {
								if r#model_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("model"));
								}
								r#model_property = Some({
									struct DeserializeWith(Vec<ModelProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "model-date-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::ModelDate => {
								if r#model_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"modelDate",
									));
								}
								r#model_date_property = Some({
									struct DeserializeWith(Vec<ModelDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "mpn-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Mpn => {
								if r#mpn_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("mpn"));
								}
								r#mpn_property = Some({
									struct DeserializeWith(Vec<MpnProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Name => {
								if r#name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("name"));
								}
								r#name_property = Some({
									struct DeserializeWith(Vec<NameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "negative-notes-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::NegativeNotes => {
								if r#negative_notes_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"negativeNotes",
									));
								}
								r#negative_notes_property = Some({
									struct DeserializeWith(Vec<NegativeNotesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "nsn-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Nsn => {
								if r#nsn_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("nsn"));
								}
								r#nsn_property = Some({
									struct DeserializeWith(Vec<NsnProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "number-of-airbags-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::NumberOfAirbags => {
								if r#number_of_airbags_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfAirbags",
									));
								}
								r#number_of_airbags_property = Some({
									struct DeserializeWith(Vec<NumberOfAirbagsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "number-of-axles-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::NumberOfAxles => {
								if r#number_of_axles_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfAxles",
									));
								}
								r#number_of_axles_property = Some({
									struct DeserializeWith(Vec<NumberOfAxlesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "number-of-doors-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::NumberOfDoors => {
								if r#number_of_doors_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfDoors",
									));
								}
								r#number_of_doors_property = Some({
									struct DeserializeWith(Vec<NumberOfDoorsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "number-of-forward-gears-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::NumberOfForwardGears => {
								if r#number_of_forward_gears_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfForwardGears",
									));
								}
								r#number_of_forward_gears_property = Some({
									struct DeserializeWith(Vec<NumberOfForwardGearsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "number-of-previous-owners-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::NumberOfPreviousOwners => {
								if r#number_of_previous_owners_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfPreviousOwners",
									));
								}
								r#number_of_previous_owners_property = Some({
									struct DeserializeWith(Vec<NumberOfPreviousOwnersProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "offers-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Offers => {
								if r#offers_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("offers"));
								}
								r#offers_property = Some({
									struct DeserializeWith(Vec<OffersProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "pattern-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Pattern => {
								if r#pattern_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"pattern",
									));
								}
								r#pattern_property = Some({
									struct DeserializeWith(Vec<PatternProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "payload-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::Payload => {
								if r#payload_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"payload",
									));
								}
								r#payload_property = Some({
									struct DeserializeWith(Vec<PayloadProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "positive-notes-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::PositiveNotes => {
								if r#positive_notes_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"positiveNotes",
									));
								}
								r#positive_notes_property = Some({
									struct DeserializeWith(Vec<PositiveNotesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "potential-action-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PotentialAction => {
								if r#potential_action_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"potentialAction",
									));
								}
								r#potential_action_property = Some({
									struct DeserializeWith(Vec<PotentialActionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "product-id-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ProductId => {
								if r#product_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"productID",
									));
								}
								r#product_id_property = Some({
									struct DeserializeWith(Vec<ProductIdProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "production-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ProductionDate => {
								if r#production_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"productionDate",
									));
								}
								r#production_date_property = Some({
									struct DeserializeWith(Vec<ProductionDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "purchase-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PurchaseDate => {
								if r#purchase_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"purchaseDate",
									));
								}
								r#purchase_date_property = Some({
									struct DeserializeWith(Vec<PurchaseDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "release-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ReleaseDate => {
								if r#release_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"releaseDate",
									));
								}
								r#release_date_property = Some({
									struct DeserializeWith(Vec<ReleaseDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "review-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Review => {
								if r#review_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("review"));
								}
								r#review_property = Some({
									struct DeserializeWith(Vec<ReviewProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "reviews-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Reviews => {
								if r#reviews_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"reviews",
									));
								}
								r#reviews_property = Some({
									struct DeserializeWith(Vec<ReviewsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "same-as-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SameAs => {
								if r#same_as_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("sameAs"));
								}
								r#same_as_property = Some({
									struct DeserializeWith(Vec<SameAsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "seating-capacity-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::SeatingCapacity => {
								if r#seating_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"seatingCapacity",
									));
								}
								r#seating_capacity_property = Some({
									struct DeserializeWith(Vec<SeatingCapacityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "size-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Size => {
								if r#size_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("size"));
								}
								r#size_property = Some({
									struct DeserializeWith(Vec<SizeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "sku-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Sku => {
								if r#sku_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("sku"));
								}
								r#sku_property = Some({
									struct DeserializeWith(Vec<SkuProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "slogan-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Slogan => {
								if r#slogan_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("slogan"));
								}
								r#slogan_property = Some({
									struct DeserializeWith(Vec<SloganProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "speed-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::Speed => {
								if r#speed_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("speed"));
								}
								r#speed_property = Some({
									struct DeserializeWith(Vec<SpeedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "steering-position-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SteeringPosition => {
								if r#steering_position_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"steeringPosition",
									));
								}
								r#steering_position_property = Some({
									struct DeserializeWith(Vec<SteeringPositionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "stupid-property-property-schema",
									feature = "attic-schema-section"
								),
								doc
							))]
							Field::StupidProperty => {
								if r#stupid_property_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"stupidProperty",
									));
								}
								r#stupid_property_property = Some({
									struct DeserializeWith(Vec<StupidPropertyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "subject-of-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SubjectOf => {
								if r#subject_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subjectOf",
									));
								}
								r#subject_of_property = Some({
									struct DeserializeWith(Vec<SubjectOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "tongue-weight-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::TongueWeight => {
								if r#tongue_weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"tongueWeight",
									));
								}
								r#tongue_weight_property = Some({
									struct DeserializeWith(Vec<TongueWeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "trailer-weight-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::TrailerWeight => {
								if r#trailer_weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"trailerWeight",
									));
								}
								r#trailer_weight_property = Some({
									struct DeserializeWith(Vec<TrailerWeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Url => {
								if r#url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("url"));
								}
								r#url_property = Some({
									struct DeserializeWith(Vec<UrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-configuration-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VehicleConfiguration => {
								if r#vehicle_configuration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleConfiguration",
									));
								}
								r#vehicle_configuration_property = Some({
									struct DeserializeWith(Vec<VehicleConfigurationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-engine-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VehicleEngine => {
								if r#vehicle_engine_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleEngine",
									));
								}
								r#vehicle_engine_property = Some({
									struct DeserializeWith(Vec<VehicleEngineProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-identification-number-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VehicleIdentificationNumber => {
								if r#vehicle_identification_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleIdentificationNumber",
									));
								}
								r#vehicle_identification_number_property = Some({
									struct DeserializeWith(
										Vec<VehicleIdentificationNumberProperty>,
									);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-interior-color-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VehicleInteriorColor => {
								if r#vehicle_interior_color_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleInteriorColor",
									));
								}
								r#vehicle_interior_color_property = Some({
									struct DeserializeWith(Vec<VehicleInteriorColorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-interior-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VehicleInteriorType => {
								if r#vehicle_interior_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleInteriorType",
									));
								}
								r#vehicle_interior_type_property = Some({
									struct DeserializeWith(Vec<VehicleInteriorTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-model-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VehicleModelDate => {
								if r#vehicle_model_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleModelDate",
									));
								}
								r#vehicle_model_date_property = Some({
									struct DeserializeWith(Vec<VehicleModelDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-seating-capacity-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VehicleSeatingCapacity => {
								if r#vehicle_seating_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleSeatingCapacity",
									));
								}
								r#vehicle_seating_capacity_property = Some({
									struct DeserializeWith(Vec<VehicleSeatingCapacityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-special-usage-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::VehicleSpecialUsage => {
								if r#vehicle_special_usage_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleSpecialUsage",
									));
								}
								r#vehicle_special_usage_property = Some({
									struct DeserializeWith(Vec<VehicleSpecialUsageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "vehicle-transmission-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VehicleTransmission => {
								if r#vehicle_transmission_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"vehicleTransmission",
									));
								}
								r#vehicle_transmission_property = Some({
									struct DeserializeWith(Vec<VehicleTransmissionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "weight-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Weight => {
								if r#weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("weight"));
								}
								r#weight_property = Some({
									struct DeserializeWith(Vec<WeightProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "weight-total-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::WeightTotal => {
								if r#weight_total_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"weightTotal",
									));
								}
								r#weight_total_property = Some({
									struct DeserializeWith(Vec<WeightTotalProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "wheelbase-property-schema",
									feature = "auto-schema-section"
								),
								doc
							))]
							Field::Wheelbase => {
								if r#wheelbase_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"wheelbase",
									));
								}
								r#wheelbase_property = Some({
									struct DeserializeWith(Vec<WheelbaseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							#[cfg(any(
								any(
									feature = "width-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Width => {
								if r#width_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("width"));
								}
								r#width_property = Some({
									struct DeserializeWith(Vec<WidthProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(Vehicle {
						#[cfg(any(
							any(
								feature = "acceleration-time-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#acceleration_time: r#acceleration_time_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#additional_property: r#additional_property_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "asin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#asin: r#asin_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#audience: r#audience_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#award: r#award_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#awards: r#awards_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "body-type-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#body_type: r#body_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#brand: r#brand_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "call-sign-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#call_sign: r#call_sign_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cargo-volume-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#cargo_volume: r#cargo_volume_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "category-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#category: r#category_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#color: r#color_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "country-of-assembly-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#country_of_assembly: r#country_of_assembly_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "country-of-last-processing-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#country_of_last_processing: r#country_of_last_processing_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#country_of_origin: r#country_of_origin_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "date-vehicle-first-registered-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#date_vehicle_first_registered: r#date_vehicle_first_registered_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "depth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#depth: r#depth_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#description: r#description_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "drive-wheel-configuration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#drive_wheel_configuration: r#drive_wheel_configuration_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "emissions-co-2-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#emissions_co_2: r#emissions_co_2_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "fuel-capacity-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#fuel_capacity: r#fuel_capacity_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "fuel-consumption-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#fuel_consumption: r#fuel_consumption_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "fuel-efficiency-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#fuel_efficiency: r#fuel_efficiency_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "fuel-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#fuel_type: r#fuel_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#funding: r#funding_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#gtin: r#gtin_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-12-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#gtin_12: r#gtin_12_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-13-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#gtin_13: r#gtin_13_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-14-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#gtin_14: r#gtin_14_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gtin-8-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#gtin_8: r#gtin_8_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-adult-consideration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_adult_consideration: r#has_adult_consideration_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-energy-consumption-details-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_energy_consumption_details: r#has_energy_consumption_details_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_measurement: r#has_measurement_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_merchant_return_policy: r#has_merchant_return_policy_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						r#has_product_return_policy: r#has_product_return_policy_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#height: r#height_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#identifier: r#identifier_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#image: r#image_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "in-product-group-with-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#in_product_group_with_id: r#in_product_group_with_id_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-accessory-or-spare-part-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_accessory_or_spare_part_for: r#is_accessory_or_spare_part_for_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-consumable-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_consumable_for: r#is_consumable_for_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_family_friendly: r#is_family_friendly_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_related_to: r#is_related_to_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-similar-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_similar_to: r#is_similar_to_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-variant-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_variant_of: r#is_variant_of_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "item-condition-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#item_condition: r#item_condition_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#keywords: r#keywords_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "known-vehicle-damages-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#known_vehicle_damages: r#known_vehicle_damages_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#logo: r#logo_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "manufacturer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#manufacturer: r#manufacturer_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#material: r#material_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "meets-emission-standard-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#meets_emission_standard: r#meets_emission_standard_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "mileage-from-odometer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#mileage_from_odometer: r#mileage_from_odometer_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "mobile-url-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#mobile_url: r#mobile_url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "model-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#model: r#model_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "model-date-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#model_date: r#model_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "mpn-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#mpn: r#mpn_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#name: r#name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "negative-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#negative_notes: r#negative_notes_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "nsn-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#nsn: r#nsn_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-airbags-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#number_of_airbags: r#number_of_airbags_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-axles-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#number_of_axles: r#number_of_axles_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-doors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#number_of_doors: r#number_of_doors_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-forward-gears-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#number_of_forward_gears: r#number_of_forward_gears_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-previous-owners-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#number_of_previous_owners: r#number_of_previous_owners_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#offers: r#offers_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#pattern: r#pattern_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "payload-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#payload: r#payload_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "positive-notes-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#positive_notes: r#positive_notes_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "product-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#product_id: r#product_id_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "production-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#production_date: r#production_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "purchase-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#purchase_date: r#purchase_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "release-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#release_date: r#release_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#review: r#review_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#reviews: r#reviews_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#same_as: r#same_as_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "seating-capacity-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#seating_capacity: r#seating_capacity_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#size: r#size_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sku-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#sku: r#sku_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#slogan: r#slogan_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "speed-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#speed: r#speed_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "steering-position-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#steering_position: r#steering_position_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "stupid-property-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						r#stupid_property: r#stupid_property_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "tongue-weight-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#tongue_weight: r#tongue_weight_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "trailer-weight-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#trailer_weight: r#trailer_weight_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#url: r#url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-configuration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vehicle_configuration: r#vehicle_configuration_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-engine-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vehicle_engine: r#vehicle_engine_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-identification-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vehicle_identification_number: r#vehicle_identification_number_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-interior-color-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vehicle_interior_color: r#vehicle_interior_color_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-interior-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vehicle_interior_type: r#vehicle_interior_type_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-model-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vehicle_model_date: r#vehicle_model_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-seating-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vehicle_seating_capacity: r#vehicle_seating_capacity_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-special-usage-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#vehicle_special_usage: r#vehicle_special_usage_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "vehicle-transmission-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vehicle_transmission: r#vehicle_transmission_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#weight: r#weight_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "weight-total-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#weight_total: r#weight_total_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "wheelbase-property-schema",
								feature = "auto-schema-section"
							),
							doc
						))]
						r#wheelbase: r#wheelbase_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#width: r#width_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "acceleration-time-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"accelerationTime",
				#[cfg(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalProperty",
				#[cfg(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalType",
				#[cfg(any(
					any(
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"aggregateRating",
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternateName",
				#[cfg(any(
					any(feature = "asin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"asin",
				#[cfg(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"audience",
				#[cfg(any(
					any(feature = "award-property-schema", feature = "general-schema-section"),
					doc
				))]
				"award",
				#[cfg(any(
					any(feature = "awards-property-schema", feature = "general-schema-section"),
					doc
				))]
				"awards",
				#[cfg(any(
					any(feature = "body-type-property-schema", feature = "auto-schema-section"),
					doc
				))]
				"bodyType",
				#[cfg(any(
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				"brand",
				#[cfg(any(
					any(
						feature = "call-sign-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"callSign",
				#[cfg(any(
					any(
						feature = "cargo-volume-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"cargoVolume",
				#[cfg(any(
					any(
						feature = "category-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"category",
				#[cfg(any(
					any(feature = "color-property-schema", feature = "general-schema-section"),
					doc
				))]
				"color",
				#[cfg(any(
					any(
						feature = "country-of-assembly-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"countryOfAssembly",
				#[cfg(any(
					any(
						feature = "country-of-last-processing-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"countryOfLastProcessing",
				#[cfg(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"countryOfOrigin",
				#[cfg(any(
					any(
						feature = "date-vehicle-first-registered-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"dateVehicleFirstRegistered",
				#[cfg(any(
					any(feature = "depth-property-schema", feature = "general-schema-section"),
					doc
				))]
				"depth",
				#[cfg(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"description",
				#[cfg(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"disambiguatingDescription",
				#[cfg(any(
					any(
						feature = "drive-wheel-configuration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"driveWheelConfiguration",
				#[cfg(any(
					any(
						feature = "emissions-co-2-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"emissionsCO2",
				#[cfg(any(
					any(
						feature = "fuel-capacity-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"fuelCapacity",
				#[cfg(any(
					any(
						feature = "fuel-consumption-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"fuelConsumption",
				#[cfg(any(
					any(
						feature = "fuel-efficiency-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"fuelEfficiency",
				#[cfg(any(
					any(
						feature = "fuel-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"fuelType",
				#[cfg(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"funding",
				#[cfg(any(
					any(feature = "gtin-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"gtin",
				#[cfg(any(
					any(
						feature = "gtin-12-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"gtin12",
				#[cfg(any(
					any(
						feature = "gtin-13-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"gtin13",
				#[cfg(any(
					any(
						feature = "gtin-14-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"gtin14",
				#[cfg(any(
					any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
					doc
				))]
				"gtin8",
				#[cfg(any(
					any(
						feature = "has-adult-consideration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasAdultConsideration",
				#[cfg(any(
					any(
						feature = "has-energy-consumption-details-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasEnergyConsumptionDetails",
				#[cfg(any(
					any(
						feature = "has-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasMeasurement",
				#[cfg(any(
					any(
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasMerchantReturnPolicy",
				#[cfg(any(
					any(
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				))]
				"hasProductReturnPolicy",
				#[cfg(any(
					any(feature = "height-property-schema", feature = "general-schema-section"),
					doc
				))]
				"height",
				#[cfg(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"identifier",
				#[cfg(any(
					any(feature = "image-property-schema", feature = "general-schema-section"),
					doc
				))]
				"image",
				#[cfg(any(
					any(
						feature = "in-product-group-with-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"inProductGroupWithID",
				#[cfg(any(
					any(
						feature = "is-accessory-or-spare-part-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isAccessoryOrSparePartFor",
				#[cfg(any(
					any(
						feature = "is-consumable-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isConsumableFor",
				#[cfg(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isFamilyFriendly",
				#[cfg(any(
					any(
						feature = "is-related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isRelatedTo",
				#[cfg(any(
					any(
						feature = "is-similar-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isSimilarTo",
				#[cfg(any(
					any(
						feature = "is-variant-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isVariantOf",
				#[cfg(any(
					any(
						feature = "item-condition-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"itemCondition",
				#[cfg(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"keywords",
				#[cfg(any(
					any(
						feature = "known-vehicle-damages-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"knownVehicleDamages",
				#[cfg(any(
					any(feature = "logo-property-schema", feature = "general-schema-section"),
					doc
				))]
				"logo",
				#[cfg(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mainEntityOfPage",
				#[cfg(any(
					any(
						feature = "manufacturer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"manufacturer",
				#[cfg(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"material",
				#[cfg(any(
					any(
						feature = "meets-emission-standard-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"meetsEmissionStandard",
				#[cfg(any(
					any(
						feature = "mileage-from-odometer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mileageFromOdometer",
				#[cfg(any(
					any(
						feature = "mobile-url-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"mobileUrl",
				#[cfg(any(
					any(feature = "model-property-schema", feature = "general-schema-section"),
					doc
				))]
				"model",
				#[cfg(any(
					any(
						feature = "model-date-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"modelDate",
				#[cfg(any(
					any(feature = "mpn-property-schema", feature = "general-schema-section"),
					doc
				))]
				"mpn",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(
						feature = "negative-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"negativeNotes",
				#[cfg(any(
					any(feature = "nsn-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"nsn",
				#[cfg(any(
					any(
						feature = "number-of-airbags-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"numberOfAirbags",
				#[cfg(any(
					any(
						feature = "number-of-axles-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"numberOfAxles",
				#[cfg(any(
					any(
						feature = "number-of-doors-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"numberOfDoors",
				#[cfg(any(
					any(
						feature = "number-of-forward-gears-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"numberOfForwardGears",
				#[cfg(any(
					any(
						feature = "number-of-previous-owners-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"numberOfPreviousOwners",
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				"offers",
				#[cfg(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"pattern",
				#[cfg(any(
					any(feature = "payload-property-schema", feature = "auto-schema-section"),
					doc
				))]
				"payload",
				#[cfg(any(
					any(
						feature = "positive-notes-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"positiveNotes",
				#[cfg(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"potentialAction",
				#[cfg(any(
					any(
						feature = "product-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"productID",
				#[cfg(any(
					any(
						feature = "production-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"productionDate",
				#[cfg(any(
					any(
						feature = "purchase-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"purchaseDate",
				#[cfg(any(
					any(
						feature = "release-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"releaseDate",
				#[cfg(any(
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				"review",
				#[cfg(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"reviews",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(
						feature = "seating-capacity-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"seatingCapacity",
				#[cfg(any(
					any(feature = "size-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"size",
				#[cfg(any(
					any(feature = "sku-property-schema", feature = "general-schema-section"),
					doc
				))]
				"sku",
				#[cfg(any(
					any(feature = "slogan-property-schema", feature = "general-schema-section"),
					doc
				))]
				"slogan",
				#[cfg(any(
					any(feature = "speed-property-schema", feature = "auto-schema-section"),
					doc
				))]
				"speed",
				#[cfg(any(
					any(
						feature = "steering-position-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"steeringPosition",
				#[cfg(any(
					any(
						feature = "stupid-property-property-schema",
						feature = "attic-schema-section"
					),
					doc
				))]
				"stupidProperty",
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subjectOf",
				#[cfg(any(
					any(
						feature = "tongue-weight-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"tongueWeight",
				#[cfg(any(
					any(
						feature = "trailer-weight-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"trailerWeight",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
				#[cfg(any(
					any(
						feature = "vehicle-configuration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"vehicleConfiguration",
				#[cfg(any(
					any(
						feature = "vehicle-engine-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"vehicleEngine",
				#[cfg(any(
					any(
						feature = "vehicle-identification-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"vehicleIdentificationNumber",
				#[cfg(any(
					any(
						feature = "vehicle-interior-color-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"vehicleInteriorColor",
				#[cfg(any(
					any(
						feature = "vehicle-interior-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"vehicleInteriorType",
				#[cfg(any(
					any(
						feature = "vehicle-model-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"vehicleModelDate",
				#[cfg(any(
					any(
						feature = "vehicle-seating-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"vehicleSeatingCapacity",
				#[cfg(any(
					any(
						feature = "vehicle-special-usage-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"vehicleSpecialUsage",
				#[cfg(any(
					any(
						feature = "vehicle-transmission-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"vehicleTransmission",
				#[cfg(any(
					any(feature = "weight-property-schema", feature = "general-schema-section"),
					doc
				))]
				"weight",
				#[cfg(any(
					any(
						feature = "weight-total-property-schema",
						feature = "auto-schema-section"
					),
					doc
				))]
				"weightTotal",
				#[cfg(any(
					any(feature = "wheelbase-property-schema", feature = "auto-schema-section"),
					doc
				))]
				"wheelbase",
				#[cfg(any(
					any(feature = "width-property-schema", feature = "general-schema-section"),
					doc
				))]
				"width",
			];
			deserializer.deserialize_struct("Vehicle", FIELDS, ClassVisitor)
		}
	}
}
