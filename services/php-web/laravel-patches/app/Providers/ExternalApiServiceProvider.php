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
     *
     * @return void
     */
    public function register()
    {
        $this->app->singleton(AstronomyClientInterface::class, function ($app) {
            $config = $app->make('config')->get('services.astronomy');

            if (!isset($config['app_id']) || !isset($config['secret'])) {
                throw new ExternalServiceException('Astronomy API credentials are not configured.');
            }

            // Create the base service that actually calls the API
            $baseService = new AstronomyDataService(
                $config['app_id'],
                $config['secret']
            );

            // Wrap it in the caching decorator
            return new RedisAstronomyCache($baseService);
        });
    }

    /**
     * Bootstrap services.
     *
     * @return void
     */
    public function boot()
    {
        //
    }
}
