#!/usr/bin/env bash
set -e

# Ensure the application key is generated
php artisan key:generate --force

# Discover and cache packages
php artisan package:discover --ansi
composer dump-autoload --optimize --ignore-platform-reqs

# Cache the configuration
php artisan config:cache

# Run database migrations
php artisan migrate --force

# Start php-fpm
php-fpm -F
