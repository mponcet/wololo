# Wololo

WOL (Wake On Lan) Slack bot, written in Rust 🚀

## Install

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

## Usage

### Run slack app

Run slack bot :
```
SLACK_APP_TOKEN=$TOKEN cargo run db.yaml
```

Database must contain records with the following format : `<slack_user_id>: <mac_address>`

Example :

```
U123456:
    mac: 01:02:03:04:05:06
U654321:
    mac: 06:05:04:03:02:01
    tcp_check_addr: 10.0.0.1:22
```
