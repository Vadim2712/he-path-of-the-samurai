<?php

namespace App\Http\Controllers;

use App\Contracts\AstronomyClientInterface;
use App\Http\Requests\GetAstroEventsRequest;
use App\Http\Resources\AstroEventResource;
use App\Jobs\UpdateAstronomyCacheJob;
use Illuminate\Http\JsonResponse;
use Illuminate\Support\Facades\Cache;

class AstroController extends Controller
{
    public function __construct(
        private readonly AstronomyClientInterface $astroClient
    ) {}

    /**
     * Handle the request for astronomical events.
     *
     * This endpoint uses an async pattern. It first checks if the data is cached.
     * If yes, it returns the data immediately.
     * If no, it dispatches a background job to fetch the data and returns a
     * 202 Accepted response, prompting the client to retry shortly.
     *
     * @param GetAstroEventsRequest $request
     * @return JsonResponse
     */
    public function events(GetAstroEventsRequest $request): JsonResponse
    {
        $validatedData = $request->validated();
        $latitude = (float)($validatedData['lat'] ?? 55.75);
        $longitude = (float)($validatedData['lon'] ?? 37.61);
        $daysToForecast = 7;

        // Using a more readable key generation method
        $cacheKey = sprintf('astronomy:events:%s:%s:%d:%s',
            number_format($latitude, 4),
            number_format($longitude, 4),
            $daysToForecast,
            now()->format('Y-m-d')
        );

        // Check if the data is already in the cache
        if (Cache::has($cacheKey)) {
            $events = Cache::get($cacheKey);

            return response()->json([
                'status' => 'COMPLETED',
                'data' => AstroEventResource::collection($events)
            ]);
        }

        // If not cached, dispatch a job and tell the client to check back
        UpdateAstronomyCacheJob::dispatch($latitude, $longitude);

        return response()->json([
            'status' => 'PROCESSING',
            'message' => 'Data is being fetched in the background. Please try again in a few seconds.',
            'retry_after' => 10 // Suggest a longer retry for cold caches
        ], 202);
    }
}
