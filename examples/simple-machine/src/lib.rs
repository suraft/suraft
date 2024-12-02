use std::collections::BTreeMap;
use std::io;

use memstore::MemLogStore;
use suraft::declare_suraft_types;
use suraft::storage::log::entry::Entry;
use suraft::storage::log::log_id::LogId;
use suraft::storage::log::log_id::LogIdOptionExt;
use suraft::storage::LogStorageExt;
use suraft::SuRaft;

pub mod logging;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cmd {
    pub key: String,
    pub value: Vec<u8>,
}

impl Cmd {
    pub fn new(key: impl ToString, value: impl ToString) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string().as_bytes().to_vec(),
        }
    }
}

declare_suraft_types!(pub Types: AppData = Cmd);

#[derive(Default)]
pub struct StateMachine {
    applied: Option<LogId>,
    data: BTreeMap<String, Vec<u8>>,
}

impl StateMachine {
    pub async fn run(
        mut self,
        su: SuRaft<Types>,
        mut log_store: MemLogStore,
    ) -> Result<(), io::Error> {
        let mut watcher = su.metrics();

        loop {
            let committed = watcher.borrow().committed;

            if committed.is_none() {
                continue;
            }

            while self.applied < committed {
                println!(
                    "StateMachine: found new committed: {:?}, sm.last_log_id: {:?}",
                    committed, self.applied
                );

                let next = self.applied.next_index();
                let entry: Entry<Types> = log_store.read_log_entry(next).await?.unwrap();

                for cmd in entry.payload {
                    self.apply(cmd);
                }
                self.applied = Some(entry.log_id);
            }

            let Ok(_x) = watcher.changed().await else {
                break;
            };
        }
        Ok(())
    }

    fn apply(&mut self, cmd: Cmd) {
        self.data.insert(cmd.key.clone(), cmd.value.clone());
        println!(
            "StateMachine: apply: {:?}; data after applying: {:?}",
            cmd, self.data
        );
    }
}
