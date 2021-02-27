#!/bin/bash

_kill_procs() {
  kill -TERM $maprenderer
  wait $maprenderer
  kill -TERM $xvfb
}

# Setup a trap to catch SIGTERM and relay it to child processes
trap _kill_procs SIGTERM

XVFB_WHD=${XVFB_WHD:-1280x720x16}

# Start Xvfb
# Xvfb :99 -ac -screen 0 $XVFB_WHD -nolisten tcp &
# Xvfb :99 -noreset -screen 0 1280x1024x24 -ac -screen -nolisten tcp &
Xvfb :99 &
xvfb=$!

export DISPLAY=:99

map-renderer $@ &
maprenderer=$!

wait $maprenderer
wait $xvfb