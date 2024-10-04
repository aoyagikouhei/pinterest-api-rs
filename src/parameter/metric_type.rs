use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
pub enum MetricTypes {
    Standard(Vec<StandardMetricType>),
    Video(Vec<VideoMetricType>),
}

impl Default for MetricTypes {
    fn default() -> Self {
        Self::Standard(vec![])
    }
}

impl std::fmt::Display for MetricTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::Standard(metric_types) => metric_types
                .iter()
                .map(|metric_type| metric_type.to_string())
                .collect::<Vec<String>>()
                .join(","),
            Self::Video(metric_types) => metric_types
                .iter()
                .map(|metric_type| metric_type.to_string())
                .collect::<Vec<String>>()
                .join(","),
        };
        write!(f, "{}", result)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StandardMetricType {
    Impression,
    OutboundClick,
    PinClick,
    Save,
    SaveRate,
    TotalComments,
    TotalReactions,
    UserFollow,
    ProfileVisit,
}

impl fmt::Display for StandardMetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Impression => write!(f, "IMPRESSION"),
            Self::OutboundClick => write!(f, "OUTBOUND_CLICK"),
            Self::PinClick => write!(f, "PIN_CLICK"),
            Self::Save => write!(f, "SAVE"),
            Self::SaveRate => write!(f, "SAVE_RATE"),
            Self::TotalComments => write!(f, "TOTAL_COMMENTS"),
            Self::TotalReactions => write!(f, "TOTAL_REACTIONS"),
            Self::UserFollow => write!(f, "USER_FOLLOW"),
            Self::ProfileVisit => write!(f, "PROFILE_VISIT"),
        }
    }
}

impl Default for StandardMetricType {
    fn default() -> Self {
        Self::Impression
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VideoMetricType {
    Impression,
    OutboundClick,
    PinClick,
    Save,
    SaveRate,
    VideoMrcView,
    Video10sView,
    Quartile95PercentView,
    VideoV50WatchTime,
    VideoStart,
    VideoAvgWatchTime,
    TotalComments,
    TotalReactions,
}

impl fmt::Display for VideoMetricType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Impression => write!(f, "IMPRESSION"),
            Self::OutboundClick => write!(f, "OUTBOUND_CLICK"),
            Self::PinClick => write!(f, "PIN_CLICK"),
            Self::Save => write!(f, "SAVE"),
            Self::SaveRate => write!(f, "SAVE_RATE"),
            Self::VideoMrcView => write!(f, "VIDEO_MRC_VIEW"),
            Self::Video10sView => write!(f, "VIDEO_10S_VIEW"),
            Self::Quartile95PercentView => write!(f, "QUARTILE_95_PERCENT_VIEW"),
            Self::VideoV50WatchTime => write!(f, "VIDEO_V50_WATCH_TIME"),
            Self::VideoStart => write!(f, "VIDEO_START"),
            Self::VideoAvgWatchTime => write!(f, "VIDEO_AVG_WATCH_TIME"),
            Self::TotalComments => write!(f, "TOTAL_COMMENTS"),
            Self::TotalReactions => write!(f, "TOTAL_REACTIONS"),
        }
    }
}

impl Default for VideoMetricType {
    fn default() -> Self {
        Self::Impression
    }
}
