use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_mode: Option<PlanModeConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanModeConfig {
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_no_warnings: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    pub agents: HashMap<String, AgentConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<GlobalConfig>,
}

impl AgentsConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AgentsConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn load_default() -> Result<Self> {
        let default_config = include_str!("../../agents-config.json");
        let config: AgentsConfig = serde_json::from_str(default_config)?;
        Ok(config)
    }

    pub fn get_agent(&self, agent_name: &str) -> Option<&AgentConfig> {
        self.agents.get(agent_name)
    }

    pub fn get_command_and_args(&self, agent_name: &str, use_plan_mode: bool) -> Option<(String, Vec<String>)> {
        let agent = self.get_agent(agent_name)?;
        
        if use_plan_mode && agent.plan_mode.is_some() {
            let plan_config = agent.plan_mode.as_ref().unwrap();
            Some((agent.command.clone(), plan_config.args.clone()))
        } else {
            Some((agent.command.clone(), agent.args.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_default_config() {
        let config = AgentsConfig::load_default().unwrap();
        assert!(config.agents.contains_key("claude"));
        assert!(config.agents.contains_key("gemini"));
    }

    #[test]
    fn test_get_agent_config() {
        let config = AgentsConfig::load_default().unwrap();
        let claude_config = config.get_agent("claude").unwrap();
        assert_eq!(claude_config.name, "Claude Code");
        assert_eq!(claude_config.command, "npx -y @anthropic-ai/claude-code@latest");
    }

    #[test]
    fn test_get_command_and_args() {
        let config = AgentsConfig::load_default().unwrap();
        let (cmd, args) = config.get_command_and_args("claude", false).unwrap();
        assert_eq!(cmd, "npx -y @anthropic-ai/claude-code@latest");
        assert!(!args.is_empty());
        
        let (cmd_plan, args_plan) = config.get_command_and_args("claude", true).unwrap();
        assert_eq!(cmd_plan, "npx -y @anthropic-ai/claude-code@latest");
        assert_ne!(args, args_plan);
    }
}