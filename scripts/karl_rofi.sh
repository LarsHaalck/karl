#!/usr/bin/env bash

KB_COPY="Alt+c"
CLIP_COMMAND="xclip -selection clipboard"

_rofi() {
    rofi -dmenu -p "Karl" \
        -kb-custom-1 "${KB_COPY}"
}


output=$(karl list --rofi | _rofi)
val=$?

# remove key
output=$(echo "${output}" | cut -d',' -f 2)
# remove outer quotation marks and one level of inner quotations
output=$(echo "${output:1:-1}" | sed 's/\\"/"/g')
# interpret backlash escapes
output=$(echo -e "${output}")

if [[ $val -eq 10 ]]; then
    echo "${output}" | eval "${CLIP_COMMAND}"
else
    xdotool type "${output}"
fi
