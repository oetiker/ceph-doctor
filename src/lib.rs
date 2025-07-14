pub mod monitor;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub mod common {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CephPgDump {
        pub pg_ready: bool,
        pub pg_map: PgMap,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PgMap {
        pub version: u64,
        pub stamp: String,
        pub last_osdmap_epoch: u64,
        pub last_pg_scan: u64,
        pub pg_stats: Vec<PgStats>,
        pub pg_stats_sum: PgStatsSum,
        pub pg_stats_delta: PgStatsDelta,
        pub osd_stats: Vec<OsdStats>,
        pub osd_stats_sum: OsdStatsSum,
        pub pool_stats: Vec<PoolStats>,
        pub pool_statfs: Vec<PoolStatfs>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PgStats {
        pub pgid: String,
        pub version: String,
        pub reported_seq: u64,
        pub reported_epoch: u64,
        pub state: String,
        pub last_fresh: String,
        pub last_change: String,
        pub last_active: String,
        pub last_peered: String,
        pub last_clean: String,
        pub last_became_active: Option<String>,
        pub last_became_peered: Option<String>,
        pub last_unstale: Option<String>,
        pub last_undegraded: Option<String>,
        pub last_fullsized: Option<String>,
        pub mapping_epoch: u64,
        pub log_start: String,
        pub ondisk_log_start: String,
        pub created: u64,
        pub last_epoch_clean: u64,
        pub parent: String,
        pub parent_split_bits: u32,
        pub last_scrub: Option<String>,
        pub last_scrub_stamp: Option<String>,
        pub last_deep_scrub: Option<String>,
        pub last_deep_scrub_stamp: Option<String>,
        pub last_clean_scrub_stamp: Option<String>,
        pub objects_scrubbed: Option<u64>,
        pub log_size: u64,
        pub log_dups_size: Option<u64>,
        pub ondisk_log_size: u64,
        pub stats_invalid: bool,
        pub dirty_stats_invalid: bool,
        pub omap_stats_invalid: bool,
        pub hitset_stats_invalid: bool,
        pub hitset_bytes_stats_invalid: bool,
        pub pin_stats_invalid: bool,
        pub manifest_stats_invalid: bool,
        pub snaptrimq_len: Option<u64>,
        pub last_scrub_duration: Option<u64>,
        pub scrub_schedule: Option<String>,
        pub scrub_duration: Option<f64>,
        pub objects_trimmed: Option<u64>,
        pub snaptrim_duration: Option<f64>,
        pub stat_sum: StatSum,
        pub up: Vec<u32>,
        pub acting: Vec<u32>,
        pub avail_no_missing: Vec<serde_json::Value>,
        pub object_location_counts: Vec<serde_json::Value>,
        pub blocked_by: Vec<serde_json::Value>,
        pub up_primary: u32,
        pub acting_primary: u32,
        pub purged_snaps: Vec<serde_json::Value>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PgStatsSum {
        pub stat_sum: StatSum,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PgStatsDelta {
        pub stat_sum: StatSum,
        pub store_stats: Statfs,
        pub log_size: u64,
        pub ondisk_log_size: u64,
        pub up: u32,
        pub acting: u32,
        pub num_store_stats: u32,
        pub stamp_delta: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct StatSum {
        pub num_bytes: i64,
        pub num_objects: i64,
        pub num_object_clones: i64,
        pub num_object_copies: i64,
        pub num_objects_missing_on_primary: i64,
        pub num_objects_missing: i64,
        pub num_objects_degraded: i64,
        pub num_objects_misplaced: i64,
        pub num_objects_unfound: i64,
        pub num_objects_dirty: i64,
        pub num_whiteouts: i64,
        pub num_read: i64,
        pub num_read_kb: i64,
        pub num_write: i64,
        pub num_write_kb: i64,
        pub num_scrub_errors: i64,
        pub num_shallow_scrub_errors: i64,
        pub num_deep_scrub_errors: i64,
        pub num_objects_recovered: i64,
        pub num_bytes_recovered: i64,
        pub num_keys_recovered: i64,
        pub num_objects_omap: i64,
        pub num_objects_hit_set_archive: i64,
        pub num_bytes_hit_set_archive: i64,
        pub num_flush: i64,
        pub num_flush_kb: i64,
        pub num_evict: i64,
        pub num_evict_kb: i64,
        pub num_promote: i64,
        pub num_flush_mode_high: i64,
        pub num_flush_mode_low: i64,
        pub num_evict_mode_some: i64,
        pub num_evict_mode_full: i64,
        pub num_objects_pinned: i64,
        pub num_legacy_snapsets: i64,
        pub num_large_omap_objects: i64,
        pub num_objects_manifest: i64,
        pub num_omap_bytes: i64,
        pub num_omap_keys: i64,
        pub num_objects_repaired: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OsdStats {
        pub osd: u32,
        pub up_from: u64,
        pub seq: u64,
        pub num_pgs: u64,
        pub kb: u64,
        pub kb_used: u64,
        pub kb_used_data: u64,
        pub kb_used_omap: u64,
        pub kb_used_meta: u64,
        pub kb_avail: u64,
        pub statfs: Statfs,
        pub hb_peers: Vec<u32>,
        pub snap_trim_queue_len: u64,
        pub num_snap_trimming: u64,
        pub num_shards_repaired: u64,
        pub op_queue_age_hist: OpQueueAgeHist,
        pub perf_stat: PerfStat,
        pub alerts: Vec<String>,
        pub network_ping_times: Vec<NetworkPingTime>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OsdStatsSum {
        pub up_from: u64,
        pub seq: u64,
        pub num_pgs: u64,
        pub num_osds: u64,
        pub num_per_pool_osds: u64,
        pub num_per_pool_omap_osds: u64,
        pub kb: u64,
        pub kb_used: u64,
        pub kb_used_data: u64,
        pub kb_used_omap: u64,
        pub kb_used_meta: u64,
        pub kb_avail: u64,
        pub statfs: Statfs,
        pub hb_peers: Vec<u32>,
        pub snap_trim_queue_len: u64,
        pub num_snap_trimming: u64,
        pub num_shards_repaired: u64,
        pub op_queue_age_hist: OpQueueAgeHist,
        pub perf_stat: PerfStat,
        pub alerts: Vec<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Statfs {
        pub total: u64,
        pub available: u64,
        pub internally_reserved: u64,
        pub allocated: u64,
        pub data_stored: u64,
        pub data_compressed: u64,
        pub data_compressed_allocated: u64,
        pub data_compressed_original: u64,
        pub omap_allocated: u64,
        pub internal_metadata: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PoolStats {
        pub poolid: u32,
        pub num_pg: u32,
        pub stat_sum: StatSum,
        pub store_stats: Statfs,
        pub log_size: u64,
        pub ondisk_log_size: u64,
        pub up: u32,
        pub acting: u32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PoolStatfs {
        pub poolid: u32,
        pub osd: u32,
        pub total: u64,
        pub available: u64,
        pub internally_reserved: u64,
        pub allocated: u64,
        pub data_stored: u64,
        pub data_compressed: u64,
        pub data_compressed_allocated: u64,
        pub data_compressed_original: u64,
        pub omap_allocated: u64,
        pub internal_metadata: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct StoreStats {
        pub bytes_total: u64,
        pub bytes_sst: u64,
        pub bytes_log: u64,
        pub bytes_misc: u64,
        pub bytes_compressed: u64,
        pub bytes_compressed_allocated: u64,
        pub bytes_compressed_original: u64,
        pub apply_latency_ms: f64,
        pub commit_latency_ms: f64,
        pub compress_rejected: u64,
        pub compress_success: u64,
        pub last_updated: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NetworkPingTime {
        pub osd: u32,
        #[serde(rename = "last update")]
        pub last_update: String,
        pub interfaces: Vec<NetworkInterface>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NetworkInterface {
        pub interface: String,
        pub average: NetworkTiming,
        pub min: NetworkTiming,
        pub max: NetworkTiming,
        pub last: f64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NetworkTiming {
        #[serde(rename = "1min")]
        pub one_min: f64,
        #[serde(rename = "5min")]
        pub five_min: f64,
        #[serde(rename = "15min")]
        pub fifteen_min: f64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OpQueueAgeHist {
        pub histogram: Vec<serde_json::Value>,
        pub upper_bound: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PerfStat {
        pub commit_latency_ms: f64,
        pub apply_latency_ms: f64,
        pub commit_latency_ns: u64,
        pub apply_latency_ns: u64,
    }

    #[derive(Debug, Default, Clone)]
    pub struct OsdDataMovement {
        pub osd_id: u32,
        pub incoming_objects: i64,
        pub outgoing_objects: i64,
        pub missing_objects: i64,
        pub excess_objects: i64,
        pub missing_objects_waiting: i64, // Objects waiting to be moved (backfill_wait)
        pub excess_objects_waiting: i64,  // Objects waiting to be moved (backfill_wait)
        pub missing_objects_active: i64,  // Objects actively being moved (backfilling)
        pub excess_objects_active: i64,   // Objects actively being moved (backfilling)
        pub incoming_predicted_time_secs: Option<u64>,
        pub outgoing_predicted_time_secs: Option<u64>,
        pub missing_objects_history: Vec<i64>, // Historical missing objects counts
        pub excess_objects_history: Vec<i64>,  // Historical excess objects counts
        pub incoming_rate: Option<f64>,        // Objects per second (incoming)
        pub outgoing_rate: Option<f64>,        // Objects per second (outgoing)
    }

    #[derive(Debug, Default, Clone)]
    pub struct InconsistentPgProgress {
        pub pgid: String,
        pub num_objects: i64,
        pub primary_osd: u32,
        pub up_osds: Vec<u32>, // All OSDs in up set
        pub state: String,
        pub objects_scrubbed: u64,
        pub scrubbed_history: Vec<u64>, // Historical scrubbed counts
        pub scrub_rate: Option<f64>,    // Objects per second
        pub eta_seconds: Option<u64>,   // Estimated seconds to completion
    }
}
