use super::types::*;
use anyhow::Result;

pub struct A2aClient {
    http: reqwest::Client,
    endpoint: String,
}

impl A2aClient {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            endpoint: endpoint.into(),
        }
    }

    /// Discover an agent by fetching its Agent Card.
    pub async fn discover(base_url: &str) -> Result<AgentCard> {
        let url = format!("{}/.well-known/agent.json", base_url.trim_end_matches('/'));
        let client = reqwest::Client::new();
        let card: AgentCard = client.get(&url).send().await?.json().await?;
        Ok(card)
    }

    /// Send a JSON-RPC request to the A2A endpoint.
    async fn rpc(&self, method: &str, params: serde_json::Value) -> Result<JsonRpcResponse> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: serde_json::json!(uuid::Uuid::new_v4().to_string()),
        };

        let response: JsonRpcResponse = self
            .http
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// Send a message to the remote agent (message/send).
    pub async fn send_message(&self, params: SendMessageParams) -> Result<Task> {
        let resp = self
            .rpc("message/send", serde_json::to_value(&params)?)
            .await?;
        if let Some(err) = resp.error {
            anyhow::bail!("A2A error {}: {}", err.code, err.message);
        }
        let task: Task =
            serde_json::from_value(resp.result.ok_or_else(|| anyhow::anyhow!("empty result"))?)?;
        Ok(task)
    }

    /// Get task status (tasks/get).
    pub async fn get_task(&self, id: &str) -> Result<Task> {
        let params = GetTaskParams { id: id.to_string() };
        let resp = self
            .rpc("tasks/get", serde_json::to_value(&params)?)
            .await?;
        if let Some(err) = resp.error {
            anyhow::bail!("A2A error {}: {}", err.code, err.message);
        }
        let task: Task =
            serde_json::from_value(resp.result.ok_or_else(|| anyhow::anyhow!("empty result"))?)?;
        Ok(task)
    }

    /// Cancel a task (tasks/cancel).
    pub async fn cancel_task(&self, id: &str) -> Result<Task> {
        let params = CancelTaskParams { id: id.to_string() };
        let resp = self
            .rpc("tasks/cancel", serde_json::to_value(&params)?)
            .await?;
        if let Some(err) = resp.error {
            anyhow::bail!("A2A error {}: {}", err.code, err.message);
        }
        let task: Task =
            serde_json::from_value(resp.result.ok_or_else(|| anyhow::anyhow!("empty result"))?)?;
        Ok(task)
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}
