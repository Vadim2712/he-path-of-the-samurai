<?php

namespace App\Http\Resources;

use Illuminate\Http\Request;
use Illuminate\Http\Resources\Json\JsonResource;

class AstroEventResource extends JsonResource
{
    /**
     * Transform the resource into an array.
     *
     * @return array<string, mixed>
     */
    public function toArray(Request $request): array
    {
        return [
            'event_name' => $this->eventName,
            'event_date' => $this->eventDate,
            'description' => $this->description,
            // Optionally include original data for debugging
            // 'raw_data' => $this->when(config('app.debug'), $this->original_data),
        ];
    }
}
