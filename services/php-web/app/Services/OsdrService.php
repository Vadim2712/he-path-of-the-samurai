<?php

namespace App\Services;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

class OsdrService
{
    /**
     * Get paginated and searchable OSDR data, with caching.
     *
     * @param int $page
     * @param int $perPage
     * @param string $searchQuery
     * @return array
     */
    public function getData(int $page, int $perPage, string $searchQuery): array
    {
        // Build a cache key that includes pagination and search query
        $cacheKey = "osdr_data:page_{$page}:per_{$perPage}:q_" . md5($searchQuery);
        $cacheDuration = 300; // 5 minutes

        return Cache::remember($cacheKey, $cacheDuration, function () use ($page, $perPage, $searchQuery) {
            $rustServiceUrl = config('services.rust_iss.base_uri');
            
            $queryParams = [
                'page' => $page,
                'per_page' => $perPage,
                'q' => $searchQuery,
            ];

            try {
                $response = Http::timeout(10)->get("$rustServiceUrl/osdr/list", $queryParams); // Corrected endpoint to /osdr/list

                if ($response->failed()) {
                    Log::error('Failed to fetch OSDR data from rust_iss.', [
                        'status' => $response->status(),
                        'body' => $response->body(),
                    ]);
                    // Return a structured error response
                    return ['error' => true, 'message' => 'Failed to load data from the service.'];
                }
                
                return $response->json();

            } catch (\Exception $e) {
                Log::critical('Could not connect to rust_iss for OSDR data.', ['error' => $e->getMessage()]);
                return ['error' => true, 'message' => 'Service is currently unavailable.'];
            }
        });
    }
}
