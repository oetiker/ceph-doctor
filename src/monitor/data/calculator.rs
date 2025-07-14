use crate::common::{CephPgDump, OsdDataMovement, InconsistentPgProgress};
use crate::monitor::state::MonitorState;
use std::collections::{HashMap, HashSet};

const HISTORY_SIZE: usize = 20;

pub fn calculate_recovery_progress_height(data: &CephPgDump) -> u16 {
    let stats_sum = &data.pg_map.pg_stats_sum.stat_sum;
    
    // Count active categories (values > 0)
    let active_categories = [
        stats_sum.num_objects_missing,
        stats_sum.num_objects_unfound,
        stats_sum.num_objects_misplaced,
        stats_sum.num_objects_degraded,
    ].iter().filter(|&&value| value > 0).count();
    
    if active_categories == 0 {
        // Just title and message: borders + title + message
        3
    } else {
        // Table height: borders + title + data rows
        // +3 for borders and title, +1 for each active category
        (3 + active_categories) as u16
    }
}

pub fn calculate_osd_data_movement(
    current_data: &CephPgDump,
    state: &mut MonitorState,
    interval: u64,
) -> HashMap<u32, OsdDataMovement> {
    let mut osd_movements = state.get_osd_movements().clone();

    // Initialize all OSDs that are currently up
    for osd_stat in &current_data.pg_map.osd_stats {
        osd_movements.entry(osd_stat.osd).or_insert_with(|| OsdDataMovement { 
            osd_id: osd_stat.osd, 
            ..Default::default() 
        });
    }

    // Reset current counts for all OSDs
    for movement in osd_movements.values_mut() {
        movement.incoming_objects = 0;
        movement.outgoing_objects = 0;
        movement.missing_objects = 0;
        movement.excess_objects = 0;
        movement.missing_objects_waiting = 0;
        movement.excess_objects_waiting = 0;
        movement.missing_objects_active = 0;
        movement.excess_objects_active = 0;
    }

    // Process ALL PGs to sum up misplaced objects per OSD
    for current_pg in &current_data.pg_map.pg_stats {
        if current_pg.stat_sum.num_objects_misplaced > 0 {
            let current_up_set: HashSet<u32> = current_pg.up.iter().cloned().collect();
            let current_acting_set: HashSet<u32> = current_pg.acting.iter().cloned().collect();

            // OSDs that are in 'up' but not in 'acting' (need data - missing objects)
            let missing_osds: Vec<u32> = current_up_set.difference(&current_acting_set).cloned().collect();
            // OSDs that are in 'acting' but not in 'up' (have excess data)
            let excess_osds: Vec<u32> = current_acting_set.difference(&current_up_set).cloned().collect();

            // Use actual misplaced object count from PG stats
            let pg_misplaced_objects = current_pg.stat_sum.num_objects_misplaced;

            // Determine state for categorization
            let is_actively_moving = current_pg.state.contains("recovering") ||
                                    current_pg.state.contains("backfilling");
            let is_waiting_to_move = current_pg.state.contains("backfill_wait");

            // Sum up misplaced objects for OSDs that need data
            for &osd_id in &missing_osds {
                let entry = osd_movements.entry(osd_id).or_insert_with(|| OsdDataMovement { osd_id, ..Default::default() });
                entry.incoming_objects += pg_misplaced_objects;
                entry.missing_objects += pg_misplaced_objects;
                
                if is_actively_moving {
                    entry.missing_objects_active += pg_misplaced_objects;
                } else if is_waiting_to_move {
                    entry.missing_objects_waiting += pg_misplaced_objects;
                }
            }

            // Sum up misplaced objects for OSDs that have excess data
            for &osd_id in &excess_osds {
                let entry = osd_movements.entry(osd_id).or_insert_with(|| OsdDataMovement { osd_id, ..Default::default() });
                entry.outgoing_objects += pg_misplaced_objects;
                entry.excess_objects += pg_misplaced_objects;
                
                if is_actively_moving {
                    entry.excess_objects_active += pg_misplaced_objects;
                } else if is_waiting_to_move {
                    entry.excess_objects_waiting += pg_misplaced_objects;
                }
            }
        }
    }

    // Update historical data and calculate ETA
    for (_osd_id, movement) in osd_movements.iter_mut() {
        // Add current missing objects count to history
        movement.missing_objects_history.push(movement.missing_objects);
        if movement.missing_objects_history.len() > HISTORY_SIZE {
            movement.missing_objects_history.remove(0);
        }

        // Add current excess objects count to history
        movement.excess_objects_history.push(movement.excess_objects);
        if movement.excess_objects_history.len() > HISTORY_SIZE {
            movement.excess_objects_history.remove(0);
        }

        // Calculate ETA using oldest vs current entry (need at least 3 data points)
        // Only calculate ETA if there are active missing objects being moved
        if movement.missing_objects_history.len() >= 3 && movement.missing_objects_active > 0 {
            let oldest_missing = movement.missing_objects_history[0];
            let current_missing = movement.missing_objects;
            let time_elapsed = (movement.missing_objects_history.len() - 1) as f64 * interval as f64;
            
            if oldest_missing > current_missing && time_elapsed > 0.0 {
                let rate = (oldest_missing - current_missing) as f64 / time_elapsed;
                movement.incoming_rate = Some(rate);
                if rate > 0.0 && current_missing > 0 {
                    let remaining_time_secs = (current_missing as f64 / rate) as u64;
                    movement.incoming_predicted_time_secs = Some(remaining_time_secs);
                }
            }
        }

        // Only calculate ETA if there are active excess objects being moved
        if movement.excess_objects_history.len() >= 3 && movement.excess_objects_active > 0 {
            let oldest_excess = movement.excess_objects_history[0];
            let current_excess = movement.excess_objects;
            let time_elapsed = (movement.excess_objects_history.len() - 1) as f64 * interval as f64;
            
            if oldest_excess > current_excess && time_elapsed > 0.0 {
                let rate = (oldest_excess - current_excess) as f64 / time_elapsed;
                movement.outgoing_rate = Some(rate);
                if rate > 0.0 && current_excess > 0 {
                    let remaining_time_secs = (current_excess as f64 / rate) as u64;
                    movement.outgoing_predicted_time_secs = Some(remaining_time_secs);
                }
            }
        }
    }

    // Store back the updated movements
    state.set_osd_movements(osd_movements.clone());
    
    osd_movements
}

pub fn calculate_inconsistent_pg_progress(
    current_data: &CephPgDump,
    state: &mut MonitorState,
    interval: u64,
) -> HashMap<String, InconsistentPgProgress> {
    let mut pg_progress = state.get_inconsistent_pg_progress().clone();
    
    // Find PGs with inconsistent state
    for pg_stat in &current_data.pg_map.pg_stats {
        if pg_stat.state.contains("inconsistent") {
            let pgid = pg_stat.pgid.clone();
            let objects_scrubbed = pg_stat.objects_scrubbed.unwrap_or(0);
            
            let entry = pg_progress.entry(pgid.clone()).or_insert_with(|| InconsistentPgProgress {
                pgid: pgid.clone(),
                num_objects: pg_stat.stat_sum.num_object_copies,
                primary_osd: pg_stat.up_primary,
                up_osds: pg_stat.up.clone(),
                state: pg_stat.state.clone(),
                objects_scrubbed,
                scrubbed_history: Vec::new(),
                scrub_rate: None,
                eta_seconds: None,
            });
            
            // Update current data
            entry.num_objects = pg_stat.stat_sum.num_object_copies;
            entry.primary_osd = pg_stat.up_primary;
            entry.up_osds = pg_stat.up.clone();
            entry.state = pg_stat.state.clone();
            entry.objects_scrubbed = objects_scrubbed;
            
            // Add to history
            entry.scrubbed_history.push(objects_scrubbed);
            if entry.scrubbed_history.len() > HISTORY_SIZE {
                entry.scrubbed_history.remove(0);
            }
            
            // Calculate rate and ETA if we have enough history
            if entry.scrubbed_history.len() >= 3 {
                let oldest_scrubbed = entry.scrubbed_history[0];
                let current_scrubbed = objects_scrubbed;
                let time_elapsed = (entry.scrubbed_history.len() - 1) as f64 * interval as f64;
                
                if current_scrubbed > oldest_scrubbed && time_elapsed > 0.0 {
                    let rate = (current_scrubbed - oldest_scrubbed) as f64 / time_elapsed;
                    entry.scrub_rate = Some(rate);
                    
                    // Calculate ETA using actual object copies
                    let estimated_total_scrubs = entry.num_objects as u64;
                    if rate > 0.0 && current_scrubbed < estimated_total_scrubs {
                        let remaining_scrubs = estimated_total_scrubs - current_scrubbed;
                        let eta_seconds = (remaining_scrubs as f64 / rate) as u64;
                        entry.eta_seconds = Some(eta_seconds);
                    }
                }
            }
        }
    }
    
    // Remove PGs that are no longer inconsistent
    let current_inconsistent_pgs: HashSet<String> = current_data.pg_map.pg_stats
        .iter()
        .filter(|pg| pg.state.contains("inconsistent"))
        .map(|pg| pg.pgid.clone())
        .collect();
    
    pg_progress.retain(|pgid, _| current_inconsistent_pgs.contains(pgid));
    
    // Store back the updated progress
    state.set_inconsistent_pg_progress(pg_progress.clone());
    
    pg_progress
}