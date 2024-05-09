use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecurringInterval {
    Month,
    Year,
}

impl RecurringInterval {
    pub fn as_str(&self) -> &str {
        match self {
            RecurringInterval::Month => "month",
            RecurringInterval::Year => "year",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPlan {
    Pro,
    Team,
}

impl SubscriptionPlan {
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionPlan::Pro => "pro",
            SubscriptionPlan::Team => "team",
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[repr(i16)]
pub enum WorkspaceSubscriptionPlan {
    Unknown = -1,

    Free = 0,
    Pro = 1,
    Team = 2,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Paused,
    Trialing,
    Unpaid,
}

#[derive(Deserialize, Debug)]
pub struct WorkspaceSubscriptionStatus {
    pub workspace_id: String,
    pub workspace_plan: WorkspaceSubscriptionPlan,
    pub recurring_interval: RecurringInterval,
    pub subscription_status: SubscriptionStatus,
}
