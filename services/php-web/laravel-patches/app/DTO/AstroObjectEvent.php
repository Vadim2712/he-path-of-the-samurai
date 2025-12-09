<?php

namespace App\DTO;

use Illuminate\Support\Carbon;

/**
 * A Data Transfer Object for a single astronomical event.
 * Ensures consistent data structure throughout the application.
 */
class AstroObjectEvent
{
    public function __construct(
        public readonly string $eventName,
        public readonly string|Carbon $eventDate,
        public readonly ?string $description = null,
        public readonly array $original_data = []
    ) {}
}
