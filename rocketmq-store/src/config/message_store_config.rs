/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::config::{broker_role::BrokerRole, flush_disk_type::FlushDiskType};

lazy_static! {
    static ref USER_HOME: String = dirs::home_dir().unwrap().to_str().unwrap().to_string();
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageStoreConfig {
    pub store_path_root_dir: String,
    pub store_path_commit_log: Option<String>,
    pub store_path_dledger_commit_log: Option<String>,
    pub store_path_epoch_file: Option<String>,
    pub store_path_broker_identity: Option<String>,
    pub read_only_commit_log_store_paths: Option<String>,
    pub mapped_file_size_commit_log: usize,
    pub compaction_mapped_file_size: usize,
    pub compaction_cq_mapped_file_size: usize,
    pub compaction_schedule_internal: usize,
    pub max_offset_map_size: usize,
    pub compaction_thread_num: usize,
    pub enable_compaction: bool,
    pub mapped_file_size_timer_log: usize,
    pub timer_precision_ms: usize,
    pub timer_roll_window_slot: usize,
    pub timer_flush_interval_ms: usize,
    pub timer_get_message_thread_num: usize,
    pub timer_put_message_thread_num: usize,
    pub timer_enable_disruptor: bool,
    pub timer_enable_check_metrics: bool,
    pub timer_intercept_delay_level: bool,
    pub timer_max_delay_sec: usize,
    pub timer_wheel_enable: bool,
    pub disappear_time_after_start: isize,
    pub timer_stop_enqueue: bool,
    pub timer_check_metrics_when: String,
    pub timer_skip_unknown_error: bool,
    pub timer_warm_enable: bool,
    pub timer_stop_dequeue: bool,
    pub timer_congest_num_each_slot: usize,
    pub timer_metric_small_threshold: usize,
    pub timer_progress_log_interval_ms: usize,
    pub store_type: String,
    pub mapped_file_size_consume_queue: usize,
    pub enable_consume_queue_ext: bool,
    pub mapped_file_size_consume_queue_ext: usize,
    pub mapper_file_size_batch_consume_queue: usize,
    pub bit_map_length_consume_queue_ext: usize,
    pub flush_interval_commit_log: usize,
    pub commit_interval_commit_log: usize,
    pub max_recovery_commit_log_files: usize,
    pub disk_space_warning_level_ratio: usize,
    pub disk_space_clean_forcibly_ratio: usize,
    pub use_reentrant_lock_when_put_message: bool,
    pub flush_commit_log_timed: bool,
    pub flush_interval_consume_queue: usize,
    pub clean_resource_interval: usize,
    pub delete_commit_log_files_interval: usize,
    pub delete_consume_queue_files_interval: usize,
    pub destroy_mapped_file_interval_forcibly: usize,
    pub redelete_hanged_file_interval: usize,
    pub delete_when: String,
    pub disk_max_used_space_ratio: usize,
    pub file_reserved_time: usize,
    pub delete_file_batch_max: usize,
    pub put_msg_index_hight_water: usize,
    pub max_message_size: usize,
    pub check_crc_on_recover: bool,
    pub flush_commit_log_least_pages: usize,
    pub commit_commit_log_least_pages: usize,
    pub flush_least_pages_when_warm_mapped_file: usize,
    pub flush_consume_queue_least_pages: usize,
    pub flush_commit_log_thorough_interval: usize,
    pub commit_commit_log_thorough_interval: usize,
    pub flush_consume_queue_thorough_interval: usize,
    pub max_transfer_bytes_on_message_in_memory: usize,
    pub max_transfer_count_on_message_in_memory: usize,
    pub max_transfer_bytes_on_message_in_disk: usize,
    pub max_transfer_count_on_message_in_disk: usize,
    pub access_message_in_memory_max_ratio: usize,
    pub message_index_enable: bool,
    pub max_hash_slot_num: usize,
    pub max_index_num: usize,
    pub max_msgs_num_batch: usize,
    pub message_index_safe: bool,
    pub ha_listen_port: usize,
    pub ha_send_heartbeat_interval: usize,
    pub ha_housekeeping_interval: usize,
    pub ha_transfer_batch_size: usize,
    pub ha_master_address: Option<String>,
    pub ha_max_gap_not_in_sync: usize,
    pub broker_role: BrokerRole,
    pub flush_disk_type: FlushDiskType,
    pub sync_flush_timeout: usize,
    pub put_message_timeout: usize,
    pub slave_timeout: usize,
    pub message_delay_level: String,
    pub flush_delay_offset_interval: usize,
    pub clean_file_forcibly_enable: bool,
    pub warm_mapped_file_enable: bool,
    pub offset_check_in_slave: bool,
    pub debug_lock_enable: bool,
    pub duplication_enable: bool,
    pub disk_fall_recorded: bool,
    pub os_page_cache_busy_timeout_mills: usize,
    pub default_query_max_num: usize,
    pub transient_store_pool_enable: bool,
    pub transient_store_pool_size: usize,
    pub fast_fail_if_no_buffer_in_store_pool: bool,
    pub enable_dledger_commit_log: bool,
    pub dledger_group: Option<String>,
    pub dledger_peers: Option<String>,
    pub dledger_self_id: Option<String>,
    pub preferred_leader_id: Option<String>,
    pub enable_batch_push: bool,
    pub enable_schedule_message_stats: bool,
    pub enable_lmq: bool,
    pub enable_multi_dispatch: bool,
    pub max_lmq_consume_queue_num: usize,
    pub enable_schedule_async_deliver: bool,
    pub schedule_async_deliver_max_pending_limit: usize,
    pub schedule_async_deliver_max_resend_num2_blocked: usize,
    pub max_batch_delete_files_num: usize,
    pub dispatch_cq_threads: usize,
    pub dispatch_cq_cache_num: usize,
    pub enable_async_reput: bool,
    pub recheck_reput_offset_from_cq: bool,
    pub max_topic_length: usize,
    pub auto_message_version_on_topic_len: bool,
    pub enabled_append_prop_crc: bool,
    pub force_verify_prop_crc: bool,
    pub travel_cq_file_num_when_get_message: usize,
    pub correct_logic_min_offset_sleep_interval: usize,
    pub correct_logic_min_offset_force_interval: usize,
    pub mapped_file_swap_enable: bool,
    pub commit_log_force_swap_map_interval: usize,
    pub commit_log_swap_map_interval: usize,
    pub commit_log_swap_map_reserve_file_num: usize,
    pub logic_queue_force_swap_map_interval: usize,
    pub logic_queue_swap_map_interval: usize,
    pub clean_swapped_map_interval: usize,
    pub logic_queue_swap_map_reserve_file_num: usize,
    pub search_bcq_by_cache_enable: bool,
    pub dispatch_from_sender_thread: bool,
    pub wake_commit_when_put_message: bool,
    pub wake_flush_when_put_message: bool,
    pub enable_clean_expired_offset: bool,
    pub max_async_put_message_requests: usize,
    pub pull_batch_max_message_count: usize,
    pub total_replicas: usize,
    pub in_sync_replicas: usize,
    pub min_in_sync_replicas: usize,
    pub all_ack_in_sync_state_set: bool,
    pub enable_auto_in_sync_replicas: bool,
    pub ha_flow_control_enable: bool,
    pub max_ha_transfer_byte_in_second: usize,
    pub ha_max_time_slave_not_catchup: usize,
    pub sync_master_flush_offset_when_startup: bool,
    pub max_checksum_range: usize,
    pub replicas_per_disk_partition: usize,
    pub logical_disk_space_clean_forcibly_threshold: f64,
    pub max_slave_resend_length: usize,
    pub sync_from_last_file: bool,
    pub async_learner: bool,
    pub max_consume_queue_scan: usize,
    pub sample_count_threshold: usize,
    pub cold_data_flow_control_enable: bool,
    pub cold_data_scan_enable: bool,
    pub data_read_ahead_enable: bool,
    pub timer_cold_data_check_interval_ms: usize,
    pub sample_steps: usize,
    pub access_message_in_memory_hot_ratio: usize,
    pub enable_build_consume_queue_concurrently: bool,
    pub batch_dispatch_request_thread_pool_nums: usize,
    pub clean_rocksdb_dirty_cq_interval_min: usize,
    pub stat_rocksdb_cq_interval_sec: usize,
    pub mem_table_flush_interval_ms: usize,
    pub real_time_persist_rocksdb_config: bool,
    pub enable_rocksdb_log: bool,
    pub topic_queue_lock_num: usize,
}
