<?php

namespace App\Services;

use App\Contracts\AstronomyClientInterface;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Cache;

class RedisAstronomyCache implements AstronomyClientInterface
{
    // Use a longer TTL, 2 hours, can be configured via constructor
    public function __construct(
        private AstronomyClientInterface $nextService,
        private int $cacheDurationSeconds = 7200
    ) {}

    public function getEvents(float $lat, float $lon, int $days): Collection
    {
        // A slightly different, more readable cache key
        $cacheKey = sprintf('astronomy:events:%s:%s:%d:%s',
            number_format($lat, 4),
            number_format($lon, 4),
            $days,
            now()->format('Y-m-d')
        );

        return Cache::remember($cacheKey, $this->cacheDurationSeconds, function () use ($lat, $lon, $days) {
            return $this->nextService->getEvents($lat, $lon, $days);
        });
    }
}
