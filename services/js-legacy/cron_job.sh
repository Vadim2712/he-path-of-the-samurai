#!/bin/bash
# This script is executed by the cron job.

# Load environment variables
if [ -f /usr/src/app/project_env.sh ]; then
  . /usr/src/app/project_env.sh
fi

# Run the node script
node /usr/src/app/index.js
