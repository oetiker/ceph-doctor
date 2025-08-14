pub mod monitor;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub mod common {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CephPgDump {
        #[serde(default)]
        pub pg_ready: bool,
        pub pg_map: PgMap,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PgMap {
        #[serde(default)]
        pub version: u64,
        pub stamp: String,
        #[serde(default)]
        pub last_osdmap_epoch: u64,
        #[serde(default)]
        pub last_pg_scan: u64,
        pub pg_stats: Vec<PgStats>,
        pub pg_stats_sum: PgStatsSum,
        #[serde(default)]
        pub pg_stats_delta: PgStatsDelta,
        pub osd_stats: Vec<OsdStats>,
        #[serde(default)]
        pub osd_stats_sum: OsdStatsSum,
        #[serde(default)]
        pub pool_stats: Vec<PoolStats>,
        #[serde(default)]
        pub pool_statfs: Vec<PoolStatfs>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PgStats {
        pub pgid: String,
        #[serde(default)]
        pub version: String,
        #[serde(default)]
        pub reported_seq: u64,
        #[serde(default)]
        pub reported_epoch: u64,
        pub state: String,
        #[serde(default)]
        pub last_fresh: String,
        #[serde(default)]
        pub last_change: String,
        #[serde(default)]
        pub last_active: String,
        #[serde(default)]
        pub last_peered: String,
        #[serde(default)]
        pub last_clean: String,
        #[serde(default)]
        pub last_became_active: Option<String>,
        #[serde(default)]
        pub last_became_peered: Option<String>,
        #[serde(default)]
        pub last_unstale: Option<String>,
        #[serde(default)]
        pub last_undegraded: Option<String>,
        #[serde(default)]
        pub last_fullsized: Option<String>,
        #[serde(default)]
        pub mapping_epoch: u64,
        #[serde(default)]
        pub log_start: String,
        #[serde(default)]
        pub ondisk_log_start: String,
        #[serde(default)]
        pub created: u64,
        #[serde(default)]
        pub last_epoch_clean: u64,
        #[serde(default)]
        pub parent: String,
        #[serde(default)]
        pub parent_split_bits: u32,
        #[serde(default)]
        pub last_scrub: Option<String>,
        #[serde(default)]
        pub last_scrub_stamp: Option<String>,
        #[serde(default)]
        pub last_deep_scrub: Option<String>,
        #[serde(default)]
        pub last_deep_scrub_stamp: Option<String>,
        #[serde(default)]
        pub last_clean_scrub_stamp: Option<String>,
        #[serde(default)]
        pub objects_scrubbed: Option<u64>,
        #[serde(default)]
        pub log_size: u64,
        #[serde(default)]
        pub log_dups_size: Option<u64>,
        #[serde(default)]
        pub ondisk_log_size: u64,
        #[serde(default)]
        pub stats_invalid: bool,
        #[serde(default)]
        pub dirty_stats_invalid: bool,
        #[serde(default)]
        pub omap_stats_invalid: bool,
        #[serde(default)]
        pub hitset_stats_invalid: bool,
        #[serde(default)]
        pub hitset_bytes_stats_invalid: bool,
        #[serde(default)]
        pub pin_stats_invalid: bool,
        #[serde(default)]
        pub manifest_stats_invalid: bool,
        #[serde(default)]
        pub snaptrimq_len: Option<u64>,
        #[serde(default)]
        pub last_scrub_duration: Option<u64>,
        #[serde(default)]
        pub scrub_schedule: Option<String>,
        #[serde(default)]
        pub scrub_duration: Option<f64>,
        #[serde(default)]
        pub objects_trimmed: Option<u64>,
        #[serde(default)]
        pub snaptrim_duration: Option<f64>,
        pub stat_sum: StatSum,
        pub up: Vec<u32>,
        pub acting: Vec<u32>,
        #[serde(default)]
        pub avail_no_missing: Vec<serde_json::Value>,
        #[serde(default)]
        pub object_location_counts: Vec<serde_json::Value>,
        #[serde(default)]
        pub blocked_by: Vec<serde_json::Value>,
        pub up_primary: u32,
        #[serde(default)]
        pub acting_primary: u32,
        #[serde(default)]
        pub purged_snaps: Vec<serde_json::Value>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PgStatsSum {
        pub stat_sum: StatSum,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
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

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct StatSum {
        pub num_bytes: i64,
        pub num_objects: i64,
        #[serde(default)]
        pub num_object_clones: i64,
        pub num_object_copies: i64,
        #[serde(default)]
        pub num_objects_missing_on_primary: i64,
        pub num_objects_missing: i64,
        pub num_objects_degraded: i64,
        pub num_objects_misplaced: i64,
        pub num_objects_unfound: i64,
        #[serde(default)]
        pub num_objects_dirty: i64,
        #[serde(default)]
        pub num_whiteouts: i64,
        #[serde(default)]
        pub num_read: i64,
        #[serde(default)]
        pub num_read_kb: i64,
        #[serde(default)]
        pub num_write: i64,
        #[serde(default)]
        pub num_write_kb: i64,
        #[serde(default)]
        pub num_scrub_errors: i64,
        #[serde(default)]
        pub num_shallow_scrub_errors: i64,
        #[serde(default)]
        pub num_deep_scrub_errors: i64,
        #[serde(default)]
        pub num_objects_recovered: i64,
        #[serde(default)]
        pub num_bytes_recovered: i64,
        #[serde(default)]
        pub num_keys_recovered: i64,
        #[serde(default)]
        pub num_objects_omap: i64,
        #[serde(default)]
        pub num_objects_hit_set_archive: i64,
        #[serde(default)]
        pub num_bytes_hit_set_archive: i64,
        #[serde(default)]
        pub num_flush: i64,
        #[serde(default)]
        pub num_flush_kb: i64,
        #[serde(default)]
        pub num_evict: i64,
        #[serde(default)]
        pub num_evict_kb: i64,
        #[serde(default)]
        pub num_promote: i64,
        #[serde(default)]
        pub num_flush_mode_high: i64,
        #[serde(default)]
        pub num_flush_mode_low: i64,
        #[serde(default)]
        pub num_evict_mode_some: i64,
        #[serde(default)]
        pub num_evict_mode_full: i64,
        #[serde(default)]
        pub num_objects_pinned: i64,
        #[serde(default)]
        pub num_legacy_snapsets: i64,
        #[serde(default)]
        pub num_large_omap_objects: i64,
        #[serde(default)]
        pub num_objects_manifest: i64,
        #[serde(default)]
        pub num_omap_bytes: i64,
        #[serde(default)]
        pub num_omap_keys: i64,
        #[serde(default)]
        pub num_objects_repaired: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct OsdStats {
        pub osd: u32,
        #[serde(default)]
        pub up_from: u64,
        #[serde(default)]
        pub seq: u64,
        #[serde(default)]
        pub num_pgs: u64,
        #[serde(default)]
        pub kb: u64,
        #[serde(default)]
        pub kb_used: u64,
        #[serde(default)]
        pub kb_used_data: u64,
        #[serde(default)]
        pub kb_used_omap: u64,
        #[serde(default)]
        pub kb_used_meta: u64,
        #[serde(default)]
        pub kb_avail: u64,
        #[serde(default)]
        pub statfs: Statfs,
        #[serde(default)]
        pub hb_peers: Vec<u32>,
        #[serde(default)]
        pub snap_trim_queue_len: u64,
        #[serde(default)]
        pub num_snap_trimming: u64,
        #[serde(default)]
        pub num_shards_repaired: u64,
        #[serde(default)]
        pub op_queue_age_hist: OpQueueAgeHist,
        #[serde(default)]
        pub perf_stat: PerfStat,
        #[serde(default)]
        pub alerts: Vec<String>,
        #[serde(default)]
        pub network_ping_times: Option<Vec<NetworkPingTime>>,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
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

    #[derive(Debug, Default, Deserialize, Serialize)]
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

    #[derive(Debug, Default, Deserialize, Serialize)]
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

    #[derive(Debug, Default, Deserialize, Serialize)]
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

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct OpQueueAgeHist {
        pub histogram: Vec<serde_json::Value>,
        pub upper_bound: u64,
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
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
