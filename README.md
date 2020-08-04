# rkeep
Persistent Rofi backend for KeePassXC in Rust, using `keepassxc-cli`.

## Todo
* Implement non-blocking clipboard clear (removed for now since `clip timeout` is blocking)

## Requirements
```
rofi
keepassxc-cli
timeout
```

## Configuration
The configuration may contain multiple sessions, in case you have multiple keepass databases. However, a client can only access one session at a time. Default location is `~/.config/rkeep/config.toml`, see `rkeep-server --help` and `rkeep-client --help`. Copy the [sample config](config.sample.toml) for reference when configuring.

**Note:** If nothing happens after entering your password, try bumping up the response timeout until it works.

### Example config

```toml
socket = "/tmp/rkeep.sock"

[[session]]
name = "mykeys" # Name of session, used in rkeep-client
database = "/path/to/my.kdbx"
alive = 1800 # Keep database unlocked for (seconds)
timeout = 1000 # Max response timeout for keepassxc-cli (milliseconds). Adjust slightly if your kdbx takes longer to decrypt
clipboard = 10 # Clear clipboard after (seconds), 0 for no never

[[session]]
name = "myotherkeys"
database = "/path/to/my.other.kdbx"
alive = 1800
timeout = 30000
clipboard = 10
```

## How to use
Run install.sh or install manually.

### Server
Run `rkeep-server` directly on startup, or as a user service. Note however that the service may need to be modified to start after your display manager, otherwise rofi may not show up. 

Personally I have no valid `After=` target for the service because I don't use a display manager, so I just add `systemctl --user start rkeep` in `.xinitrc` and omit enabling the service.

### Client
Set up a keybind or a shortcut to run `rkeep-client -s mykeys`.

