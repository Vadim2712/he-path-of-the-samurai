#!/bin/bash

# Default to running once every 5 minutes if not specified
CRON_SCHEDULE=${GEN_PERIOD_SEC:-300}

# Create a cron job file
echo "*/${CRON_SCHEDULE} * * * * node /usr/src/app/index.js >> /var/log/cron.log 2>&1" > /etc/cron.d/legacy-cron

# Give execution rights on the cron job
chmod 0644 /etc/cron.d/legacy-cron

# Create the log file to be able to run tail
touch /var/log/cron.log

# Run the command on container startup
echo "Service started. Running once now and then every ${CRON_SCHEDULE} seconds."
node /usr/src/app/index.js

# Start cron in the foreground
cron && tail -f /var/log/cron.log
