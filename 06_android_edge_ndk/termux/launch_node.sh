#!/system/bin/sh
export PATH=/data/data/com.termux/files/usr/bin:/data/data/com.termux/files/usr/bin/applets:$PATH
export LD_LIBRARY_PATH=/data/data/com.termux/files/usr/lib
export HOME=/data/data/com.termux/files/home
export TMPDIR=/data/data/com.termux/files/usr/tmp

# If deno is in Termux path, execute directly
if command -v deno >/dev/null 2>&1; then
    deno run --allow-net --allow-read --allow-write /data/local/tmp/run_pixel10_edge_node.ts 127.0.0.1
else
    /data/data/com.termux/files/usr/bin/deno run --allow-net --allow-read --allow-write /data/local/tmp/run_pixel10_edge_node.ts 127.0.0.1
fi