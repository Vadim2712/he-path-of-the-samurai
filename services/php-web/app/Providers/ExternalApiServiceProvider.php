<?php

namespace App\Providers;

use App\Contracts\AstronomyClientInterface;
use App\Services\AstronomyDataService;
use App\Services\RedisAstronomyCache;
use Illuminate\Support\ServiceProvider;
use App\Exceptions\ExternalServiceException;

class ExternalApiServiceProvider extends ServiceProvider
{
    /**
     * Register services.
     */
    public function register(): void
    {
        $this->app->singleton(AstronomyClientInterface::class, function ($app) {
            // Get credentials from the config file
            $appId = config('services.astronomy.app_id');
            $secret = config('services.astronomy.secret');

            if (!$appId || !$secret) {
                // This will prevent the application from starting if the credentials are not set
                throw new ExternalServiceException('Astronomy API credentials are not configured.');
            }

            // Create the base data service
            $baseService = new AstronomyDataService($appId, $secret);

            // Decorate it with the Redis caching layer
            return new RedisAstronomyCache($baseService);
        });
    }

    /**
     * Bootstrap services.
     */
    public function boot(): void
    {
        //
    }
}
