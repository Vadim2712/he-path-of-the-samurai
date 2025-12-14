#!/bin/bash

# Save environment variables to a file for the cron job, adding the 'export' keyword
echo "Exporting environment variables for cron"
printenv | grep -E 'PGHOST|PGPORT|PGDATABASE|PGUSER|PGPASSWORD|CSV_OUT_DIR|GEN_PERIOD_SEC' | sed 's/^\(.*\)$/export \1/g' > /usr/src/app/project_env.sh

# Default to running once every 5 minutes if not specified
CRON_SCHEDULE_MIN=$((GEN_PERIOD_SEC / 60))
if [ "$CRON_SCHEDULE_MIN" -eq "0" ]; then
    CRON_SCHEDULE_MIN=5
fi

# Create a cron job file to run the wrapper script
echo "*/${CRON_SCHEDULE_MIN} * * * * /usr/src/app/cron_job.sh >> /var/log/cron.log 2>&1" > /etc/cron.d/legacy-cron

# Give execution rights on the cron job
chmod 0644 /etc/cron.d/legacy-cron

# Create the log file to be able to run tail
touch /var/log/cron.log

# Run the command on container startup
echo "Service started. Running once now and then every ${CRON_SCHEDULE_MIN} minutes."
/usr/src/app/cron_job.sh

# Start cron in the foreground
echo "Starting cron daemon"
cron && tail -f /var/log/cron.log
