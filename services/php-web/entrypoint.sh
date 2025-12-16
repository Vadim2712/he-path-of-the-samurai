#!/usr/bin/env bash
set -e

# Set correct permissions for storage and cache
chown -R www-data:www-data /var/www/html/storage /var/www/html/bootstrap/cache
chmod -R 775 /var/www/html/storage /var/www/html/bootstrap/cache

# Discover and cache packages
php artisan package:discover --ansi
composer dump-autoload --optimize --ignore-platform-reqs

# Cache the configuration
php artisan config:cache

# Run database migrations
php artisan migrate --force

# Start php-fpm
php-fpm -F
