

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndividualType {
    Hobbyist,
    Professional,
    Freelancer,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualClassification {
    pub subtype: IndividualType,
}

pub struct ClassificationEvidence;
pub struct ContextAwareLicense;
pub struct EnterpriseIndicator;
pub struct EnterprisePricingModel;
pub struct FunctionUsagePattern;
pub struct HardwareProfile;
pub struct IndividualIndicator;
pub struct IntegrationComplexity;
pub struct NetworkProfile;
pub struct OrganizationSize;
pub struct PrimaryUseCase;
pub struct PurposeAnalysis;
pub struct EnterpriseTax;
pub struct EntropyBenefit;
pub struct EntropyMultipliers;
pub struct EntropyProfile;
pub struct HyperscaleIndicator;
pub struct OrganizationScale;
pub struct PricingRecommendation;
pub struct PricingTier;
pub struct ProgressiveMultipliers;
pub struct ScaleEvidence;
pub struct UsageBasedPricing;
pub struct VolumePricingTier;
