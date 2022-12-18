# wololo
Wake your devices over lan, in Rust 🚀

## Usage (slack)

### 1. Create slack app with the following manifest
```
display_information:
  name: Wololo
features:
  bot_user:
    display_name: Wololo
    always_online: true
  slash_commands:
    - command: /wololo
      description: Call Wololo bot
      should_escape: false
oauth_config:
  scopes:
    bot:
      - commands
settings:
  interactivity:
    is_enabled: true
  org_deploy_enabled: false
  socket_mode_enabled: true
  token_rotation_enabled: false
```

### 2. Create an app token with scope *connections:write*

### 3. Run slack app

```
SLACK_APP_TOKEN=$TOKEN cargo run service slack
```