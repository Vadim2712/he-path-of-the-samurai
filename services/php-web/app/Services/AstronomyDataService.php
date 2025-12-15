<?php
namespace App\Services;

use App\Contracts\AstronomyClientInterface;
use App\DTO\AstroObjectEvent;
use App\Exceptions\ExternalServiceException;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Http;
use Illuminate\Http\Client\ConnectionException;
use Illuminate\Support\Facades\Log;

class AstronomyDataService implements AstronomyClientInterface
{
    private const BASE_URL = 'https://api.astronomyapi.com/api/v2/bodies/events/Sun';

    public function __construct(
        private readonly string $appId,
        private readonly string $appSecret
    ) {}

    public function getEvents(float $lat, float $lon, int $days): Collection
    {
        $queryParams = [
            'latitude' => (string)$lat,
            'longitude' => (string)$lon,
            'from_date' => now('UTC')->toDateString(),
            'to_date' => now('UTC')->addDays($days)->toDateString(),
            'elevation' => 0,
            'time' => now('UTC')->format('H:i:s')
        ];

        try {
            $apiResponse = Http::withBasicAuth($this->appId, $this->appSecret)
                ->timeout(15)
                ->retry(3, 200)
                ->withHeaders(['User-Agent' => 'Cassiopeia-Project/1.0'])
                ->get(self::BASE_URL, $queryParams);

            if ($apiResponse->failed()) {
                Log::error('Astronomy API request failed.', [
                    'status' => $apiResponse->status(),
                    'body' => $apiResponse->body()
                ]);
                throw new ExternalServiceException(
                    "AstronomyAPI request failed with status: " . $apiResponse->status()
                );
            }

            $jsonData = $apiResponse->json();
            
            if (!isset($jsonData['data']['table']['rows'])) {
                Log::warning('Astronomy API response has an unexpected structure.', ['response' => $jsonData]);
                return collect([]);
            }

            $eventRows = $jsonData['data']['table']['rows'];
            $processedEvents = new Collection();

            foreach ($eventRows as $row) {
                try {
                    $bodyName = $row['entry']['name'] ?? 'Unknown';
                    foreach ($row['cells'] as $cell) {
                        $eventType = $cell['type'] ?? 'general';
                        
                        $eventDate = data_get($cell, 'eventHighlights.peak.date')
                            ?? data_get($cell, 'eventHighlights.partialStart.date')
                            // Adding fallback for different structures
                            ?? data_get($cell, 'rise.date')
                            ?? data_get($cell, 'set.date')
                            ?? now('UTC')->toIso8601String();
                        
                        $eventTypeName = ucfirst(str_replace('_', ' ', $eventType));
                        
                        $processedEvents->push(new AstroObjectEvent(
                            eventName: "{$bodyName} - {$eventTypeName}",
                            eventDate: $eventDate,
                            description: "Astronomical event of type {$eventTypeName} for body {$bodyName}.",
                            original_data: $cell
                        ));
                    }
                } catch (\Throwable $t) {
                    Log::error('Failed to process an individual astronomy event row.', ['row' => $row, 'error' => $t->getMessage()]);
                    // Continue to next row
                }
            }

            return $processedEvents;

        } catch (ConnectionException $e) {
            Log::critical('Could not connect to AstronomyAPI.', ['error' => $e->getMessage()]);
            throw new ExternalServiceException("The Astronomy API is currently unavailable.", 503, $e);
        }
    }
}
