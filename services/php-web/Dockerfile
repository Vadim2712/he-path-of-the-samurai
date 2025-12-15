# Stage 1: Build stage with composer
FROM composer:2.7.2 AS vendor

WORKDIR /app

COPY database/ database/
COPY composer.json composer.lock ./

RUN composer install --no-interaction --no-scripts --no-autoloader


# Stage 2: Final application image
FROM php:8.3-fpm-alpine

WORKDIR /var/www/html

# Install system dependencies
RUN apk add --no-cache bash curl git unzip rsync postgresql-dev

# Copy application files from the current directory to the container
COPY . .

# Copy vendor directory from the build stage
COPY --from=vendor /app/vendor/ ./vendor/

# Set permissions
RUN mkdir -p /var/www/html/storage/framework/sessions \
    && mkdir -p /var/www/html/storage/framework/views \
    && mkdir -p /var/www/html/storage/framework/cache/data \
    && mkdir -p /var/www/html/storage/logs \
    && mkdir -p /var/www/html/bootstrap/cache \
    && chown -R www-data:www-data /var/www/html/storage /var/www/html/bootstrap/cache

# Copy and set executable for entrypoint
COPY entrypoint.sh /usr/local/bin/php_entrypoint.sh
RUN chmod +x /usr/local/bin/php_entrypoint.sh

# Expose port 9000 and start php-fpm
EXPOSE 9000
CMD ["/usr/local/bin/php_entrypoint.sh"]
