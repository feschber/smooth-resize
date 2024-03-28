# Smooth Resizing in Sway

A sway-IPC hack for smooth window resizing in sway.


https://github.com/feschber/smooth-resize/assets/40996949/f48381d8-6d59-4e66-9c73-97692df767ce


## Installation

```sh
git clone https://github.com/feschber/smooth-resize
cd smooth-resize
cargo build --release
sudo mkdir -p /usr/local/bin/
sudo cp target/release/smooth-resize /usr/local/bin/
```

## Sway Config

```sh
# --no-repeat so only one instance is spawned
bindsym --no-repeat $mod+Ctrl+$left  exec "smooth-resize shrink width 10"
bindsym --no-repeat $mod+Ctrl+$up    exec "smooth-resize grow height 10"
bindsym --no-repeat $mod+Ctrl+$down  exec "smooth-resize shrink height 10"
bindsym --no-repeat $mod+Ctrl+$right exec "smooth-resize grow width 10"

# important -> otherwise resizing won't stop
bindsym --release $mod+Ctrl+$left  exec "killall smooth-resize"
bindsym --release $mod+Ctrl+$up    exec "killall smooth-resize"
bindsym --release $mod+Ctrl+$down  exec "killall smooth-resize"
bindsym --release $mod+Ctrl+$right exec "killall smooth-resize"
```
