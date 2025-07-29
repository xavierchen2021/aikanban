# AI Agents 配置指南

Vibe Kanban 现在支持通过配置文件来自定义不同 AI Agent 的启动方式。

## 配置文件位置

配置文件位于项目根目录：`agents-config.json`

## 配置文件结构

配置文件包含两个主要部分：

### 1. agents 对象
为每个支持的 AI Agent 定义启动配置：

- **claude**: Claude Code 默认模式
- **claude-plan**: Claude Code 计划模式
- **gemini**: Google Gemini CLI
- **amp**: Anthropic Amp
- **aider**: Aider AI 编程助手
- **codex**: OpenAI Codex
- **ccr**: Claude Code Review
- **sst-opencode**: SST OpenCode
- **charm-opencode**: Charm OpenCode

### 2. global 对象
定义全局配置：
- **node_no_warnings**: 是否禁用 Node.js 警告
- **shell_command**: shell 命令（默认为 auto 自动检测）

## 配置示例

```json
{
  "agents": {
    "claude": {
      "name": "Claude Code",
      "command": "npx -y @anthropic-ai/claude-code@latest",
      "args": ["-p", "--dangerously-skip-permissions"],
      "streaming": false
    }
  },
  "global": {
    "node_no_warnings": true
  }
}
```

## 自定义启动方式

### 修改现有 Agent

要修改 Claude Code 的启动方式，只需编辑 `agents.claude` 部分：

```json
{
  "agents": {
    "claude": {
      "name": "My Custom Claude",
      "command": "node",
      "args": ["/path/to/custom/claude.js", "--custom-flag"]
    }
  }
}
```

### 使用本地开发版本

如果你想使用本地开发的 Claude Code：

```json
{
  "agents": {
    "claude": {
      "name": "Local Claude",
      "command": "node",
      "args": ["/Users/me/dev/claude-code/dist/index.js", "--verbose"]
    }
  }
}
```

### 环境变量

你可以在命令中使用环境变量：

```json
{
  "agents": {
    "claude": {
      "name": "Claude with Proxy",
      "command": "npx",
      "args": ["-y", "@anthropic-ai/claude-code@latest", "--proxy", "http://proxy.company.com:8080"]
    }
  }
}
```

## 故障排除

### 配置文件加载失败

如果配置文件加载失败，系统会回退到默认的硬编码配置。

### 验证配置

启动应用后，检查控制台日志确认配置是否生效：
- 成功加载配置时会显示 "Loaded agent config from..."
- 如果配置文件无效，会显示警告信息并使用默认配置

### 重置为默认

要重置为默认配置，只需删除或重命名 `agents-config.json` 文件即可。

## 支持的 Agent 列表

当前支持的 Agent：
1. **claude** - Claude Code 默认模式
2. **claude-plan** - Claude Code 计划模式
3. **gemini** - Google Gemini CLI
4. **amp** - Anthropic Amp
5. **aider** - Aider AI 编程助手
6. **codex** - OpenAI Codex
7. **ccr** - Claude Code Review
8. **sst-opencode** - SST OpenCode
9. **charm-opencode** - Charm OpenCode