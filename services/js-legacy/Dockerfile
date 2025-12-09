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

# Copy the entrypoint script and make it executable
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Run the entrypoint script
ENTRYPOINT ["/entrypoint.sh"]
