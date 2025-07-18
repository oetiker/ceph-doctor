use crate::common::{InconsistentPgProgress, OsdDataMovement};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RecoveryData {
    pub objects: i64,
    pub bytes: i64,
}

#[derive(Debug, Clone)]
pub struct CommandError {
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub scroll_offset: u16,
}

#[derive(Debug, Default)]
pub struct MonitorState {
    recovery_history: HashMap<String, Vec<RecoveryData>>,
    osd_movements: HashMap<u32, OsdDataMovement>,
    inconsistent_pg_progress: HashMap<String, InconsistentPgProgress>,
    command_error_popup: Option<CommandError>,
}

impl MonitorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_recovery_history(&self, category: &str) -> Option<&Vec<RecoveryData>> {
        self.recovery_history.get(category)
    }

    pub fn get_recovery_history_mut(&mut self, category: &str) -> &mut Vec<RecoveryData> {
        self.recovery_history
            .entry(category.to_string())
            .or_default()
    }

    pub fn add_recovery_data(&mut self, category: &str, data: RecoveryData, max_history: usize) {
        let history = self.get_recovery_history_mut(category);
        history.push(data);
        if history.len() > max_history {
            history.remove(0);
        }
    }

    pub fn get_osd_movements(&self) -> &HashMap<u32, OsdDataMovement> {
        &self.osd_movements
    }

    pub fn get_osd_movements_mut(&mut self) -> &mut HashMap<u32, OsdDataMovement> {
        &mut self.osd_movements
    }

    pub fn set_osd_movements(&mut self, movements: HashMap<u32, OsdDataMovement>) {
        self.osd_movements = movements;
    }

    pub fn get_inconsistent_pg_progress(&self) -> &HashMap<String, InconsistentPgProgress> {
        &self.inconsistent_pg_progress
    }

    pub fn get_inconsistent_pg_progress_mut(
        &mut self,
    ) -> &mut HashMap<String, InconsistentPgProgress> {
        &mut self.inconsistent_pg_progress
    }

    pub fn set_inconsistent_pg_progress(
        &mut self,
        progress: HashMap<String, InconsistentPgProgress>,
    ) {
        self.inconsistent_pg_progress = progress;
    }

    pub fn clear_recovery_history(&mut self) {
        self.recovery_history.clear();
    }

    pub fn clear_osd_movements(&mut self) {
        self.osd_movements.clear();
    }

    pub fn clear_inconsistent_pg_progress(&mut self) {
        self.inconsistent_pg_progress.clear();
    }

    pub fn get_command_error_popup(&self) -> Option<&CommandError> {
        self.command_error_popup.as_ref()
    }

    pub fn set_command_error_popup(&mut self, error: CommandError) {
        self.command_error_popup = Some(error);
    }

    pub fn clear_command_error_popup(&mut self) {
        self.command_error_popup = None;
    }

    pub fn has_command_error_popup(&self) -> bool {
        self.command_error_popup.is_some()
    }

    pub fn scroll_popup_up(&mut self) {
        if let Some(error) = &mut self.command_error_popup {
            error.scroll_offset = error.scroll_offset.saturating_sub(1);
        }
    }

    pub fn scroll_popup_down(&mut self) {
        if let Some(error) = &mut self.command_error_popup {
            error.scroll_offset = error.scroll_offset.saturating_add(1);
        }
    }
}
