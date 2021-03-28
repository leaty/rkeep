# rkeep
Persistent Rofi backend for KeePass in Rust.

## Todo
* Docs

## Requirements
```
rofi
```

## Configuration
The configuration may contain multiple sessions, in case you have multiple keepass databases. Default location is `~/.config/rkeep/config.toml`, see `rkeepd --help` and `rkeep --help`. Copy the [sample config](config.sample.toml) for reference when configuring.

### Example config

```toml
socket = "/tmp/rkeep.sock"

[[session]]
name = "mykeys" # Name of session
database = "/path/to/my.kdbx"
alive = 1800 # Keep database unlocked for (seconds)
clipboard = 10 # Clear clipboard after (seconds)

[[session]]
name = "myotherkeys"
database = "/path/to/my.other.kdbx"
alive = 1800
clipboard = 10
```

## How to use
Run install.sh or install manually.

### Server
Run `rkeepd` directly on startup, or as a user service. Note however that the service may need to be modified to start after your display manager, otherwise rofi may not show up. 

Personally I have no valid `After=` target for the service because I don't use a display manager, so I just add `systemctl --user start rkeepd` in `.xinitrc` and omit enabling the service.

### Client
Set up a keybind or a shortcut to run e.g. `rkeep -s mykeys`, or run it manually.

