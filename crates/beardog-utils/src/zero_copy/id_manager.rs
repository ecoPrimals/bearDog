

use super::cow_string::IdString;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};
use tracing::{debug, trace};

pub struct IdManager {

    id_cache: Arc<RwLock<HashMap<String, Weak<str>>>>,

    stats: IdManagerStats,

    last_cleanup: Arc<RwLock<Instant>>,
}
#[derive(Debug, Default)]
pub struct IdManagerStats {

    pub cache_hits: std::sync::atomic::AtomicU64,

    pub cache_misses: std::sync::atomic::AtomicU64,

    pub ids_created: std::sync::atomic::AtomicU64,

    pub ids_cleaned: std::sync::atomic::AtomicU64,}

impl IdManager {

    pub fn new() -> Self {
        Self {
            id_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            stats: IdManagerStats::default(),
            last_cleanup: Arc::new(RwLock::new(Instant::now())),
        }
    }

    pub fn get_shared_id(&self, id: &str) -> Arc<str> {

        {
            let cache = self.id_cache.read();
            if let Some(weak_id) = cache.get(id) {
                if let Some(arc_id) = weak_id.upgrade() {
                    self.stats
                        .cache_hits
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    trace!("ID cache hit for: {}", id);
                    return arc_id;
                }
            }

        self.stats
            .cache_misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            .ids_created
        let arc_id: Arc<str> = Arc::from(id);
        let weak_id = Arc::downgrade(&arc_id);
            let mut cache = self.id_cache.write();
            cache.insert(id.to_string(), weak_id);
        trace!("Created new shared ID: {}", id);
        arc_id

    pub fn create_request_id(&self, prefix: &str) -> IdString {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let random = rand::random::<u32>();
        let id = format!("{prefix}_{timestamp:x}_{random:x}");
        IdString::new(id)

    pub fn create_task_id(&self, prefix: &str, sequence: u64) -> IdString {
        IdString::from_sequential(prefix, sequence)

    pub fn create_node_id(&self, hostname: &str, instance: u32) -> IdString {
        let id = format!("{hostname}_{instance:04}");

    pub fn cleanup_expired(&self) {
        let mut last_cleanup = self.last_cleanup.write();
        let now = Instant::now();
        if now.duration_since(*last_cleanup) < Duration::from_secs(30) {
            return; // Cleanup at most every 30 seconds
        let mut cleaned_count = 0;
            cache.retain(|_key, weak_id| {
                if weak_id.strong_count() == 0 {
                    cleaned_count += 1;
                    false
                } else {
                    true
            });
        if cleaned_count > 0 {
            self.stats
                .ids_cleaned
                .fetch_add(cleaned_count, std::sync::atomic::Ordering::Relaxed);
            debug!("Cleaned up {} expired ID references", cleaned_count);
        *last_cleanup = now;

    pub fn get_stats(&self) -> &IdManagerStats {
        &self.stats

    pub fn cache_size(&self) -> usize {
        self.id_cache.read().len()
impl Default for IdManager {}

    fn default() -> Self {
        Self::new()

static GLOBAL_ID_MANAGER: std::sync::OnceLock<IdManager> = std::sync::OnceLock::new();

pub fn global_id_manager() -> &'static IdManager {
    GLOBAL_ID_MANAGER.get_or_init(IdManager::new)

pub fn shared_request_id(request_id: &str) -> Arc<str> {
    global_id_manager().get_shared_id(request_id)

pub fn shared_task_id(task_id: &str) -> Arc<str> {
    global_id_manager().get_shared_id(task_id)

pub fn shared_node_id(node_id: &str) -> Arc<str> {
    global_id_manager().get_shared_id(node_id)

pub fn shared_operation_id(operation_id: &str) -> Arc<str> {
    global_id_manager().get_shared_id(operation_id)

pub fn generate_request_id(prefix: &str) -> IdString {
    global_id_manager().create_request_id(prefix)

pub fn generate_task_id(prefix: &str, sequence: u64) -> IdString {
    global_id_manager().create_task_id(prefix, sequence)

pub fn generate_node_id(hostname: &str, instance: u32) -> IdString {
    global_id_manager().create_node_id(hostname, instance)

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommonIdPattern {

    Request(IdString),

    Task(IdString),

    Node(IdString),

    Operation(IdString),

    Session(IdString),

    User(IdString),

    Generic(IdString),}

impl CommonIdPattern {

    pub fn from_string(id: &str) -> Self {
        let id_str = IdString::new(id);
        Self::from_id_string(id_str)

    pub fn from_id_string(id: IdString) -> Self {
        let s = id.as_str();
        if s.starts_with("req_") || s.contains("request") {
            Self::Request(id)
        } else if s.starts_with("task_") {
            Self::Task(id)
        } else if s.starts_with("node_") {
            Self::Node(id)
        } else if s.starts_with("op_") || s.starts_with("operation_") {
            Self::Operation(id)
        } else if s.starts_with("sess_") || s.starts_with("session_") {
            Self::Session(id)
        } else if s.starts_with("user_") {
            Self::User(id)
        } else {
            Self::Generic(id)

    pub fn id(&self) -> &IdString {
        match self {
            | Self::Task(id)
            | Self::Node(id)
            | Self::Operation(id)
            | Self::Session(id)
            | Self::User(id)
            | Self::Generic(id) => id,

    pub fn as_str(&self) -> &str {
        self.id().as_str()

    pub fn pattern_type(&self) -> &'static str {
            Self::Request(_) => "request",
            Self::Task(_) => "task",
            Self::Node(_) => "node",
            Self::Operation(_) => "operation",
            Self::Session(_) => "session",
            Self::User(_) => "user",
            Self::Generic(_) => "generic",

    pub fn estimated_frequency(&self) -> u64 {
            Self::Request(_) => 10, // Example frequency
            Self::Task(_) => 5,
            Self::Node(_) => 3,
            Self::Operation(_) => 2,
            Self::Session(_) => 1,
            Self::User(_) => 1,
            Self::Generic(_) => 1,
impl std::fmt::Display for CommonIdPattern {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
impl AsRef<str> for CommonIdPattern {}

    fn as_ref(&self) -> &str {
        self.as_str()

pub struct IdBatch {
    ids: Vec<CommonIdPattern>,}

impl IdBatch {

        Self { ids: Vec::new() }

    pub fn add_id<S: Into<String>>(&mut self, id: S) {
        let pattern = CommonIdPattern::from_string(id.into());
        self.ids.push(pattern);

    pub fn process(self) -> Vec<CommonIdPattern> {

        let mut deduped_ids = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();
        for id in self.ids {
            let id_string = id.as_str();
            if !seen_ids.contains(id_string) {
                seen_ids.insert(id_string.to_string());
                deduped_ids.push(id);

        deduped_ids.sort_by(|a, b| b.estimated_frequency().cmp(&a.estimated_frequency()));
        deduped_ids

    pub fn len(&self) -> usize {
        self.ids.len()

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
impl Default for IdBatch {

fn extract_sequential_number(id: &str) -> Option<u64> {

    let mut last_digit_end = 0;
    let mut in_digit_sequence = false;
    for (i, c) in id.char_indices() {
        if c.is_ascii_digit() {
            if !in_digit_sequence {

                in_digit_sequence = true;
            last_digit_end = i + c.len_utf8();
        } else if in_digit_sequence {

            in_digit_sequence = false;

    if last_digit_end > 0 {
        let mut start = 0;
        for (i, c) in id.char_indices() {
            if c.is_ascii_digit() {
                start = i;
                break;

        for i in (0..last_digit_end).rev() {
            if let Some(c) = id.chars().nth(i) {
                if !c.is_ascii_digit() {
                    start = i + c.len_utf8();
                    break;
        if let Ok(num) = id[start..last_digit_end].parse::<u64>() {
            return Some(num);
    None
