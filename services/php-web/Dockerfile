# Stage 1: Build stage with composer
FROM php:8.4-cli-alpine AS vendor
RUN apk add --no-cache git \
    && curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer \
    && composer self-update --2

WORKDIR /app

COPY database/ database/
COPY composer.json composer.lock ./

RUN composer install --no-interaction --no-scripts --no-autoloader


# Stage 2: Final application image
FROM php:8.4-fpm-alpine

WORKDIR /var/www/html

# Install system dependencies
RUN apk add --no-cache bash curl git unzip rsync postgresql-dev $PHPIZE_DEPS \
    && curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer \
    && composer self-update --2 \
    && docker-php-ext-install pdo pdo_pgsql pgsql \
    && pecl install redis \
    && docker-php-ext-enable pdo_pgsql redis

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
