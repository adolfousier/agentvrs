use crate::agent::{AgentId, AgentKind, AgentMessage};
use crate::api::observability::{ActivityEntry, ActivityKind, HeartbeatInfo, TaskRecord};
use crate::world::Position;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use std::collections::VecDeque;
use uuid::Uuid;

/// SQLite persistence layer for agents, messages, activity, tasks, and heartbeats.
pub struct Database {
    conn: Connection,
}

/// Minimal agent data for restoring from database.
pub struct AgentRow {
    pub id: AgentId,
    pub name: String,
    pub kind: AgentKind,
    pub position: Position,
    pub color_index: u8,
}

impl Database {
    pub fn open() -> Result<Self> {
        let db_path = db_path();
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    /// In-memory database for testing.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS agents (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                kind TEXT NOT NULL,
                pos_x INTEGER NOT NULL,
                pos_y INTEGER NOT NULL,
                color_index INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_agent TEXT NOT NULL,
                to_agent TEXT NOT NULL,
                text TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS activity_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                agent_id TEXT NOT NULL,
                kind TEXT NOT NULL,
                detail TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS tasks (
                task_id TEXT NOT NULL,
                agent_id TEXT NOT NULL,
                state TEXT NOT NULL,
                submitted_at TEXT NOT NULL,
                last_updated TEXT NOT NULL,
                response_summary TEXT,
                PRIMARY KEY (task_id, agent_id)
            );
            CREATE TABLE IF NOT EXISTS heartbeats (
                agent_id TEXT PRIMARY KEY,
                last_seen TEXT NOT NULL,
                status TEXT NOT NULL,
                metadata TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_messages_to ON messages(to_agent);
            CREATE INDEX IF NOT EXISTS idx_activity_agent ON activity_log(agent_id);
            CREATE INDEX IF NOT EXISTS idx_activity_ts ON activity_log(timestamp);",
        )?;
        Ok(())
    }

    // ── Agents ───────────────────────────────────────────────────────────

    pub fn save_agent(&self, agent: &crate::agent::Agent) -> Result<()> {
        let kind_json = serde_json::to_string(&agent.kind)?;
        tracing::debug!(
            "DB: saving agent '{}' (id={}, pos=({},{}))",
            agent.name, agent.id, agent.position.x, agent.position.y
        );
        self.conn.execute(
            "INSERT OR REPLACE INTO agents (id, name, kind, pos_x, pos_y, color_index) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                agent.id.0.to_string(),
                agent.name,
                kind_json,
                agent.position.x as i32,
                agent.position.y as i32,
                agent.color_index as i32,
            ],
        )?;
        tracing::debug!("DB: agent '{}' saved successfully", agent.name);
        Ok(())
    }

    pub fn load_agents(&self) -> Result<Vec<AgentRow>> {
        tracing::debug!("DB: loading agents from database");
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, kind, pos_x, pos_y, color_index FROM agents")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i32>(3)?,
                row.get::<_, i32>(4)?,
                row.get::<_, i32>(5)?,
            ))
        })?;

        let mut agents = Vec::new();
        for row in rows {
            let (id_str, name, kind_json, pos_x, pos_y, color_index) = row?;
            let uuid = Uuid::parse_str(&id_str)?;
            let kind: AgentKind = serde_json::from_str(&kind_json)?;
            agents.push(AgentRow {
                id: AgentId(uuid),
                name,
                kind,
                position: Position::new(pos_x as u16, pos_y as u16),
                color_index: color_index as u8,
            });
        }
        tracing::debug!("DB: loaded {} agents", agents.len());
        Ok(agents)
    }

    pub fn remove_agent(&self, id: &AgentId) -> Result<()> {
        self.conn.execute(
            "DELETE FROM agents WHERE id = ?1",
            params![id.0.to_string()],
        )?;
        Ok(())
    }

    pub fn update_agent_position(&self, id: &AgentId, pos: Position) -> Result<()> {
        self.conn.execute(
            "UPDATE agents SET pos_x = ?1, pos_y = ?2 WHERE id = ?3",
            params![pos.x as i32, pos.y as i32, id.0.to_string()],
        )?;
        Ok(())
    }

    // ── Messages ─────────────────────────────────────────────────────────

    pub fn save_message(&self, msg: &AgentMessage) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (from_agent, to_agent, text, timestamp) \
             VALUES (?1, ?2, ?3, ?4)",
            params![
                msg.from.0.to_string(),
                msg.to.0.to_string(),
                msg.text,
                msg.timestamp.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn load_messages_for(
        &self,
        agent_id: &AgentId,
        limit: usize,
    ) -> Result<VecDeque<AgentMessage>> {
        let mut stmt = self.conn.prepare(
            "SELECT from_agent, to_agent, text, timestamp FROM messages \
             WHERE to_agent = ?1 ORDER BY id DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![agent_id.0.to_string(), limit as i64], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;

        let mut messages = VecDeque::new();
        for row in rows {
            let (from_str, to_str, text, ts_str) = row?;
            let from = AgentId(Uuid::parse_str(&from_str)?);
            let to = AgentId(Uuid::parse_str(&to_str)?);
            let timestamp: DateTime<Utc> = ts_str.parse()?;
            messages.push_front(AgentMessage {
                from,
                to,
                text,
                timestamp,
            });
        }
        Ok(messages)
    }

    pub fn clear_messages_for(&self, agent_id: &AgentId) -> Result<usize> {
        let count = self.conn.execute(
            "DELETE FROM messages WHERE to_agent = ?1",
            params![agent_id.0.to_string()],
        )?;
        Ok(count)
    }

    // ── Activity ─────────────────────────────────────────────────────────

    pub fn save_activity(&self, agent_id: AgentId, entry: &ActivityEntry) -> Result<()> {
        tracing::debug!("DB: saving activity for agent {} — {:?}: {}", agent_id, entry.kind, entry.detail);
        let kind_str = serde_json::to_string(&entry.kind)?;
        self.conn.execute(
            "INSERT INTO activity_log (agent_id, kind, detail, timestamp) \
             VALUES (?1, ?2, ?3, ?4)",
            params![
                agent_id.0.to_string(),
                kind_str,
                entry.detail,
                entry.timestamp.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn load_activity(&self, agent_id: &AgentId, limit: usize) -> Result<Vec<ActivityEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT kind, detail, timestamp FROM activity_log \
             WHERE agent_id = ?1 ORDER BY id DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![agent_id.0.to_string(), limit as i64], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut entries = Vec::new();
        for row in rows {
            let (kind_str, detail, ts_str) = row?;
            let kind: ActivityKind = serde_json::from_str(&kind_str)?;
            let timestamp: DateTime<Utc> = ts_str.parse()?;
            entries.push(ActivityEntry {
                timestamp,
                kind,
                detail,
            });
        }
        entries.reverse(); // oldest first
        Ok(entries)
    }

    // ── Tasks ────────────────────────────────────────────────────────────

    pub fn save_task(&self, agent_id: AgentId, task: &TaskRecord) -> Result<()> {
        tracing::debug!("DB: saving task '{}' state={} for agent {}", task.task_id, task.state, agent_id);
        self.conn.execute(
            "INSERT OR REPLACE INTO tasks \
             (task_id, agent_id, state, submitted_at, last_updated, response_summary) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                task.task_id,
                agent_id.0.to_string(),
                task.state,
                task.submitted_at.to_rfc3339(),
                task.last_updated.to_rfc3339(),
                task.response_summary,
            ],
        )?;
        Ok(())
    }

    pub fn delete_task(&self, agent_id: AgentId, task_id: &str) -> Result<bool> {
        tracing::debug!("DB: deleting task '{}' for agent {}", task_id, agent_id);
        let changes = self.conn.execute(
            "DELETE FROM tasks WHERE agent_id = ?1 AND task_id = ?2",
            params![agent_id.0.to_string(), task_id],
        )?;
        Ok(changes > 0)
    }

    pub fn load_tasks(&self, agent_id: &AgentId, limit: usize) -> Result<Vec<TaskRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT task_id, state, submitted_at, last_updated, response_summary FROM tasks \
             WHERE agent_id = ?1 ORDER BY last_updated DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![agent_id.0.to_string(), limit as i64], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })?;

        let mut tasks = Vec::new();
        for row in rows {
            let (task_id, state, sub_str, upd_str, summary) = row?;
            let submitted_at: DateTime<Utc> = sub_str.parse()?;
            let last_updated: DateTime<Utc> = upd_str.parse()?;
            tasks.push(TaskRecord {
                task_id,
                submitted_at,
                state,
                last_updated,
                response_summary: summary,
            });
        }
        tasks.reverse();
        Ok(tasks)
    }

    // ── Heartbeats ───────────────────────────────────────────────────────

    pub fn save_heartbeat(&self, agent_id: AgentId, hb: &HeartbeatInfo) -> Result<()> {
        let metadata_json = hb
            .metadata
            .as_ref()
            .map(|m| serde_json::to_string(m).unwrap_or_default());
        self.conn.execute(
            "INSERT OR REPLACE INTO heartbeats (agent_id, last_seen, status, metadata) \
             VALUES (?1, ?2, ?3, ?4)",
            params![
                agent_id.0.to_string(),
                hb.last_seen.to_rfc3339(),
                hb.status,
                metadata_json,
            ],
        )?;
        Ok(())
    }

    pub fn load_heartbeats(&self) -> Result<std::collections::HashMap<AgentId, HeartbeatInfo>> {
        let mut stmt = self
            .conn
            .prepare("SELECT agent_id, last_seen, status, metadata FROM heartbeats")?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        })?;

        let mut map = std::collections::HashMap::new();
        for row in rows {
            let (id_str, seen_str, status, meta_str) = row?;
            let uuid = Uuid::parse_str(&id_str).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            });
            if let Ok(uuid) = uuid {
                let last_seen: DateTime<Utc> = seen_str.parse().unwrap_or_else(|_| Utc::now());
                let metadata = meta_str.and_then(|s| serde_json::from_str(&s).ok());
                map.insert(
                    AgentId(uuid),
                    HeartbeatInfo {
                        last_seen,
                        status,
                        metadata,
                    },
                );
            }
        }
        Ok(map)
    }

    // ── Cleanup ──────────────────────────────────────────────────────────

    /// Remove all data associated with an agent.
    pub fn purge_agent(&self, id: &AgentId) -> Result<()> {
        tracing::debug!("DB: purging all data for agent {}", id);
        let id_str = id.0.to_string();
        self.conn
            .execute("DELETE FROM agents WHERE id = ?1", params![id_str])?;
        self.conn
            .execute("DELETE FROM messages WHERE to_agent = ?1", params![id_str])?;
        self.conn.execute(
            "DELETE FROM activity_log WHERE agent_id = ?1",
            params![id_str],
        )?;
        self.conn
            .execute("DELETE FROM tasks WHERE agent_id = ?1", params![id_str])?;
        self.conn.execute(
            "DELETE FROM heartbeats WHERE agent_id = ?1",
            params![id_str],
        )?;
        Ok(())
    }
}

fn db_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    std::path::PathBuf::from(home)
        .join(".config")
        .join("agentverse")
        .join("agentverse.db")
}
