# Dockerfile for js-legacy service

# Use an official Node.js runtime as a parent image
FROM node:18-slim

# Install cron and other necessary tools
RUN apt-get update && apt-get -y install cron

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install any needed packages
RUN npm install

# Copy the rest of the application code
COPY . .

# Copy the entrypoint and cron job scripts and make them executable
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
COPY cron_job.sh /usr/src/app/cron_job.sh
RUN chmod +x /usr/src/app/cron_job.sh

# Run the entrypoint script
ENTRYPOINT ["/entrypoint.sh"]
