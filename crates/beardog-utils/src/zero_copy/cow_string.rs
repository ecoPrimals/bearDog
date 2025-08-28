

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ZeroCopyString {

    Static(&'static str),

    Shared(Arc<str>),

    Owned(String),
}
impl ZeroCopyString {

    pub fn from_static(s: &'static str) -> Self {
        Self::Static(s)
    }

    pub fn from_shared(s: Arc<str>) -> Self {
        Self::Shared(s)

    pub fn from_owned(s: &str) -> Self {
        if s.len() <= 3 || is_likely_id(&s) {

            Self::Shared(Arc::from(s.as_str()))
        } else {
            Self::Owned(s)
        }

    pub fn from_str<S: AsRef<str>>(s: S) -> Self {
        let s_ref = s.as_ref();

        if let Some(common) = CommonString::from_str(s_ref) {
            return Self::Static(common.as_str());

        if s_ref.len() <= 3 || is_likely_id(s_ref) || is_common_value(s_ref) {
            Self::Shared(Arc::from(s_ref))
            Self::Owned(s_ref.to_string())

    pub fn as_str(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Shared(s) => s.as_ref(),
            Self::Owned(s) => s.as_str(),

    pub fn into_owned(self) -> String {
            Self::Static(s) => s.to_string(),
            Self::Shared(s) => s.to_string(),
            Self::Owned(s) => s,

    pub fn memory_category(&self) -> &'static str {
            Self::Static(_) => "zero-allocation",
            Self::Shared(_) => "shared-reference",
            Self::Owned(_) => "owned-allocation",

    pub fn is_zero_allocation(&self) -> bool {
        matches!(self, Self::Static(_))

    pub fn push_str(&mut self, s: &str) {
            Self::Static(current) => {
                let mut owned = current.to_string();
                owned.push_str(s);
                *self = Self::Owned(owned);
            }
            Self::Shared(current) => {
            Self::Owned(current) => {
                current.push_str(s);
impl AsRef<str> for ZeroCopyString {}

    fn as_ref(&self) -> &str {
        self.as_str()
impl fmt::Display for ZeroCopyString {}

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
impl From<&'static str> for ZeroCopyString {}

    fn from(s: &'static str) -> Self {
        Self::from_static(s)
impl From<String> for ZeroCopyString {}

    fn from(s: &str) -> Self {
        Self::from_owned(s)
impl From<Arc<str>> for ZeroCopyString {}

    fn from(s: Arc<str>) -> Self {
        Self::from_shared(s)
impl<'a> From<Cow<'a, str>> for ZeroCopyString {}

    fn from(cow: Cow<'a, str>) -> Self {
        match cow {
            Cow::Borrowed(s) => Self::from_str(s),
            Cow::Owned(s) => Self::from_owned(s),}

impl Serialize for ZeroCopyString {}

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
impl<'de> Deserialize<'de> for ZeroCopyString {}

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        D: serde::Deserializer<'de>,
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_owned(s))

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdString(ZeroCopyString);
impl IdString {

    pub fn new<S: AsRef<str>>(id: S) -> Self {
        Self(ZeroCopyString::from_str(id))

    pub fn from_uuid(uuid: &str) -> Self {

        Self(ZeroCopyString::Shared(Arc::from(uuid)))

    pub fn from_sequential(prefix: &str, number: u64) -> Self {
        let id = format!("{prefix}{number:06}");
        Self(ZeroCopyString::from_owned(id))

        self.0.as_str()

    pub fn to_string(&self) -> String {
        self.0.as_str().to_string()

    pub fn is_uuid_like(&self) -> bool {
        let s = self.as_str();
        s.len() == 36 && s.chars().filter(|&c| c == '-').count() == 4

    pub fn is_sequential(&self) -> bool {
        s.len() >= 3
            && s[..3].chars().all(|c| c.is_ascii_alphabetic())
            && s[3..].chars().all(|c| c.is_ascii_digit())
impl AsRef<str> for IdString {}

impl fmt::Display for IdString {
impl From<String> for IdString {
        Self::new(s)}

impl From<&str> for IdString {
    fn from(s: &str) -> Self {

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommonString {

    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,

    ApplicationJson,
    TextPlain,
    ApplicationOctetStream,
    TextHtml,

    Success,
    Error,
    Pending,
    Completed,
    Failed,
    Active,
    Inactive,

    Beardog,
    Songbird,
    Nestgate,
    Squirrel,
    Toadstool,
    Biomeos,

    True,
    False,
    Yes,
    No,

    Api,
    Metrics,
    Health,
    Admin,}

impl CommonString {

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "get" => Some(Self::Get),
            "post" => Some(Self::Post),
            "put" => Some(Self::Put),
            "delete" => Some(Self::Delete),
            "patch" => Some(Self::Patch),
            "options" => Some(Self::Options),
            "head" => Some(Self::Head),
            "application/json" => Some(Self::ApplicationJson),
            "text/plain" => Some(Self::TextPlain),
            "application/octet-stream" => Some(Self::ApplicationOctetStream),
            "text/html" => Some(Self::TextHtml),
            "success" => Some(Self::Success),
            "error" => Some(Self::Error),
            "pending" => Some(Self::Pending),
            "completed" => Some(Self::Completed),
            "failed" => Some(Self::Failed),
            "active" => Some(Self::Active),
            "inactive" => Some(Self::Inactive),

            "security_provider" => Some(Self::SecurityProvider),
            "communication_mesh" => Some(Self::CommunicationMesh),
            "storage_services" => Some(Self::StorageServices),
            "ai_intelligence" => Some(Self::AIIntelligence),
            "compute_orchestration" => Some(Self::ComputeOrchestration),
            "system_integration" => Some(Self::SystemIntegration),
            "true" => Some(Self::True),
            "false" => Some(Self::False),
            "yes" => Some(Self::Yes),
            "no" => Some(Self::No),
            "api" => Some(Self::Api),
            "metrics" => Some(Self::Metrics),
            "health" => Some(Self::Health),
            "admin" => Some(Self::Admin),
            _ => None,

    pub fn as_str(self) -> &'static str {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Patch => "PATCH",
            Self::Options => "OPTIONS",
            Self::Head => "HEAD",
            Self::ApplicationJson => "application/json",
            Self::TextPlain => "text/plain",
            Self::ApplicationOctetStream => "application/octet-stream",
            Self::TextHtml => "text/html",
            Self::Success => "success",
            Self::Error => "error",
            Self::Pending => "pending",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::SecurityProvider => "security_provider",
            Self::CommunicationMesh => "communication_mesh",
            Self::StorageServices => "storage_services",
            Self::AIIntelligence => "ai_intelligence",
            Self::ComputeOrchestration => "compute_orchestration",
            Self::SystemIntegration => "system_integration",
            Self::True => "true",
            Self::False => "false",
            Self::Yes => "yes",
            Self::No => "no",
            Self::Api => "api",
            Self::Metrics => "metrics",
            Self::Health => "health",
            Self::Admin => "admin",
impl fmt::Display for CommonString {

fn is_likely_id(s: &str) -> bool {
    s.ends_with("_id") || s.starts_with("id_") || 
    (s.len() == 36 && s.chars().filter(|&c| c == '-').count() == 4) || // UUID
    (s.len() >= 6 && s.chars().all(|c| c.is_ascii_alphanumeric())) || // Hash-like
    s.chars().all(|c| c.is_ascii_digit()) // Numeric ID

fn is_static_string(s: &str) -> bool {
    CommonString::from_str(s).is_some()

fn is_common_value(s: &str) -> bool {
    s.len() <= 10
        || s.chars().all(|c| c.is_ascii_alphanumeric())
        || s.starts_with("http")
        || s.ends_with(".local")
        || s.ends_with(".com")
