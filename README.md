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

## Example
With `keys` as session name.

![2021-04-03-1617473165_screenshot_2560x1440](https://user-images.githubusercontent.com/4429327/113487462-8a4b0e00-94b8-11eb-8a07-1c48c04eff26.png)

![2021-04-03-1617473299_screenshot_2560x1440](https://user-images.githubusercontent.com/4429327/113487465-91721c00-94b8-11eb-9434-7050bb53d378.png)
