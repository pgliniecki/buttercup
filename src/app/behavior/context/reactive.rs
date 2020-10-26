use std::sync::{Mutex, MutexGuard, PoisonError};

use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use futures::future::AbortHandle;

pub struct ReactiveService {

    abort_handles: DashMap<i32, AbortEntry>

}

pub enum ReactiveServiceError {

    AbortEntryNotFound(i32),
    AbortEntryLockError(i32, String)

}

impl ReactiveService {

    pub fn new() -> ReactiveService {
        ReactiveService { abort_handles: DashMap::new() }
    }

    pub fn cleanup_nodes(&self, bt_node_ids: &Vec<i32>) {
        for id in bt_node_ids {
            self.abort_handles.remove(id);
        }
    }

    pub fn initialize_nodes(&self,
                            bt_node_ids: &Vec<i32>) {
        for id in bt_node_ids {
            self.abort_handles.insert(*id, AbortEntry::new(*id));
        }
    }

    pub fn register(&self,
                    bt_node_id: &i32,
                    handle: AbortHandle) -> Result<(), ReactiveServiceError> {
        match self.abort_handles.get(bt_node_id) {
            None => Result::Err(ReactiveServiceError::AbortEntryNotFound(*bt_node_id)),
            Some(entry) => match entry.push(handle) {
                Ok(_) => Result::Ok(()),
                Err(err) => Result::Err(err)
            }
        }
    }

    pub fn abort(&self,
                 bt_node_id: &i32) -> Result<(), ReactiveServiceError> {
        match self.abort_handles.get(bt_node_id) {
            None => Result::Err(ReactiveServiceError::AbortEntryNotFound(*bt_node_id)),
            Some(entry) => match entry.abort() {
                Ok(_) => Result::Ok(()),
                Err(err) => Result::Err(err)
            }
        }
    }

}

impl Default for ReactiveService {
    fn default() -> Self {
        ReactiveService::new()
    }
}


struct AbortEntry {

    bt_node_id: i32,
    handles: Mutex<Vec<AbortHandle>>

}

impl AbortEntry {

    fn new(bt_node_id: i32) -> AbortEntry {
        AbortEntry {
            bt_node_id,
            handles: Mutex::new(Vec::new())
        }
    }

    fn push(&self, handle: AbortHandle) -> Result<(), ReactiveServiceError> {
        match self.handles.lock() {
            Ok(mut handlers) => {
                handlers.push(handle);
                Result::Ok(())
            },
            Err(err) =>
                Result::Err(
                    ReactiveServiceError::AbortEntryLockError(self.bt_node_id, err.to_string()))
        }
    }

    fn abort(&self) -> Result<(), ReactiveServiceError> {
        match self.handles.lock() {
            Ok(handlers) => {
                for handle in handlers.iter() {
                    handle.abort();
                }
                Result::Ok(())
            },
            Err(err) =>
                Result::Err(
                    ReactiveServiceError::AbortEntryLockError(self.bt_node_id, err.to_string()))
        }
    }

}