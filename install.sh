cargo build --release
cp rkeep.service ~/.config/systemd/user/
sudo cp target/release/{rkeep-server,rkeep-client} /usr/bin/
