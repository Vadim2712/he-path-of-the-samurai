<?php

namespace App\Contracts;

use Illuminate\Support\Collection;

/**
 * Interface for a client that retrieves astronomical event data.
 */
interface AstronomyClientInterface
{
    /**
     * Fetches a collection of astronomical events.
     *
     * @param float $lat
     * @param float $lon
     * @param int $days
     * @return Collection
     */
    public function getEvents(float $lat, float $lon, int $days): Collection;
}
