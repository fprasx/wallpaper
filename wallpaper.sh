#!/bin/sh

# If you try to change the wallpaper to a file with the same name, it seems like
# it uses a cached version. Thus, we just make a new file with a different name
# and delete the old one.
set -u
rm ./wallpaper*.png
WALLPAPER_FILE="wallpaper-$(date "+%Y-%m-%d-%H-%M-%S").png"
cargo run -- "$WALLPAPER_FILE"
automator -i "$WALLPAPER_FILE" wp.workflow
