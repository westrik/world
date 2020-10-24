#!/usr/bin/env bash

set -euo pipefail

web_scripts_dir="$( cd "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
web_root_dir=$(dirname "$web_scripts_dir")
icons_dir="$web_root_dir/src/static/img/icons"
icon_components_dir="$icons_dir/__generated"
compressed_icons_dir="$icons_dir/__compressed"

mkdir -p "$icon_components_dir"

for icon_path in "$icons_dir"/*.svg; do
  icon_filename=$(basename "$icon_path")
  icon_name="${icon_filename%.*}"
  icon_component_name="$(echo "$icon_name" | perl -pe 's/(^|-)./uc($&)/ge;s/-//g')Icon"
  compressed_icon_path="$compressed_icons_dir/$icon_filename"
  icon_component_path="$icon_components_dir/$icon_component_name.tsx"

  if [ ! -f "$compressed_icon_path" ]; then
    echo "Compressing $icon_name"
    yarn svgo -i "$icon_path" -o "$compressed_icon_path" --datauri=base64
  fi
  compressed_icon_data=$(cat $compressed_icon_path)
  if [ ! -f "$icon_component_path" ]; then
    echo "Creating \`$icon_component_name\` component"
    cat > "$icon_component_path" << EOF
import { h } from 'preact';

export default function $icon_component_name(): h.JSX.Element {
    return <img src="$compressed_icon_data" />;
}
EOF
  fi
done
