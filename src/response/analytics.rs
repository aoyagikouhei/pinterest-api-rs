use serde::{Deserialize, Serialize};
use std::collections::HashMap as Map;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataStatus {
    Processing,
    Ready,
    Estimate,
    BeforeBusinessCreated,
    BeforeDataRetentionPeriod,
    BeforePinDataRetentionPeriod,
    BeforeMetricStartDate,
    BeforeCoreMetricStartDate,
    BeforePinFormatMetricStartDate,
    BeforeAudienceMetricStartDate,
    BeforeAudienceMonthlyMetricStartDate,
    BeforeVideoMetricStartDate,
    BeforeConversionMetricStartDate,
    PurchasersMetricSmallerThanThreshold,
    InBadTagDate,
    BeforePublishedMetricStartDate,
    BeforeAssistMetricStartDate,
    BeforePinCreated,
    BeforeAccountClaimed,
    BeforeDemographicFiltersStartDate,
    AudienceSegmentSmallerThanThreshold,
    AudienceTotalSmallerThanThreshold,
    BeforeProductGroupFilterStartDate,
}

// Privacyの表示を設定
impl std::fmt::Display for DataStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Processing => write!(f, "PROCESSING"),
            Self::Ready => write!(f, "READY"),
            Self::Estimate => write!(f, "ESTIMATE"),
            Self::BeforeBusinessCreated => write!(f, "BEFORE_BUSINESS_CREATED"),
            Self::BeforeDataRetentionPeriod => write!(f, "BEFORE_DATA_RETENTION_PERIOD"),
            Self::BeforePinDataRetentionPeriod => write!(f, "BEFORE_PIN_DATA_RETENTION_PERIOD"),
            Self::BeforeMetricStartDate => write!(f, "BEFORE_METRIC_START_DATE"),
            Self::BeforeCoreMetricStartDate => write!(f, "BEFORE_CORE_METRIC_START_DATE"),
            Self::BeforePinFormatMetricStartDate => {
                write!(f, "BEFORE_PIN_FORMAT_METRIC_START_DATE")
            }
            Self::BeforeAudienceMetricStartDate => write!(f, "BEFORE_AUDIENCE_METRIC_START_DATE"),
            Self::BeforeAudienceMonthlyMetricStartDate => {
                write!(f, "BEFORE_AUDIENCE_MONTHLY_METRIC_START_DATE")
            }
            Self::BeforeVideoMetricStartDate => write!(f, "BEFORE_VIDEO_METRIC_START_DATE"),
            Self::BeforeConversionMetricStartDate => {
                write!(f, "BEFORE_CONVERSION_METRIC_START_DATE")
            }
            Self::PurchasersMetricSmallerThanThreshold => {
                write!(f, "PURCHASERS_METRIC_SMALLER_THAN_THRESHOLD")
            }
            Self::InBadTagDate => write!(f, "IN_BAD_TAG_DATE"),
            Self::BeforePublishedMetricStartDate => write!(f, "BEFORE_PUBLISHED_METRIC_START_DATE"),
            Self::BeforeAssistMetricStartDate => write!(f, "BEFORE_ASSIST_METRIC_START_DATE"),
            Self::BeforePinCreated => write!(f, "BEFORE_PIN_CREATED"),
            Self::BeforeAccountClaimed => write!(f, "BEFORE_ACCOUNT_CLAIMED"),
            Self::BeforeDemographicFiltersStartDate => {
                write!(f, "BEFORE_DEMOGRAPHIC_FILTERS_START_DATE")
            }
            Self::AudienceSegmentSmallerThanThreshold => {
                write!(f, "AUDIENCE_SEGMENT_SMALLER_THAN_THRESHOLD")
            }
            Self::AudienceTotalSmallerThanThreshold => {
                write!(f, "AUDIENCE_TOTAL_SMALLER_THAN_THRESHOLD")
            }
            Self::BeforeProductGroupFilterStartDate => {
                write!(f, "BEFORE_PRODUCT_GROUP_FILTER_START_DATE")
            }
        }
    }
}

// Privacyのデフォルト値を設定
impl Default for DataStatus {
    fn default() -> Self {
        Self::Processing
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DailyMetrics {
    pub data_status: DataStatus,
    pub date: String,
    pub metrics: serde_json::Value,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AnalyticsItem {
    pub lifetime_metrics: serde_json::Value,
    pub daily_metrics: Vec<DailyMetrics>,
    pub summary_metrics: serde_json::Value,

    #[serde(flatten, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, serde_json::Value>,
}

pub type Analytics = Map<String, AnalyticsItem>;
