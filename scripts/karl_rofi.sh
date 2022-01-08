#!/usr/bin/env bash

cache_file="${XDG_CACHE_HOME-$HOME/.cache}/karl/clips.data"
output=$(cat "${cache_file}" | jq ".named[], .unnamed[]" | rofi -dmenu -p "Karl")
output=$(echo "${output:1:-1}" | sed 's/\\"/"/g')
output=$(echo -e "${output}")
xdotool type "${output}"
