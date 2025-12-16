<?php

namespace App\Contracts;

use Illuminate\Support\Collection;

interface AstronomyClientInterface
{
    /**
     * Get a collection of celestial events.
     *
     * @param float $lat
     * @param float $lon
     * @param int $days
     * @return Collection
     */
    public function getEvents(float $lat, float $lon, int $days): Collection;
}
