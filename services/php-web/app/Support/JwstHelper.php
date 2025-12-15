<?php

namespace App\Support;

use Illuminate\Support\Facades\Http;
use Illuminate\Http\Client\PendingRequest;

/**
 * A helper class for cleaner JWST API interactions.
 */
class JwstHelper
{
    protected PendingRequest $client;
    protected string $baseUrl;

    public function __construct()
    {
        $this->baseUrl = env('JWST_API_URL', 'https://www.stsci.edu/jwst/science-execution/program-information.json');
        $apiKey = env('JWST_API_KEY', ''); // Not currently used by this public API

        $this->client = Http::baseUrl($this->baseUrl)
            ->timeout(15)
            ->retry(2, 100) // Retry twice, wait 100ms between attempts
            ->withHeaders([
                'User-Agent' => 'Cassiopeia-Project/1.0',
                'Accept' => 'application/json',
                // Add Authorization header if the API requires a key
                // 'Authorization' => 'Bearer ' . $apiKey,
            ]);
    }

    /**
     * Perform a GET request to the JWST API.
     *
     * @param string $path The API path to request.
     * @param array $query An associative array of query parameters.
     * @return array
     */
    public function get(string $path, array $query = []): array
    {
        // This is a simplified client. A real-world scenario would have more robust error handling.
        $response = $this->client->get($path, $query);

        if ($response->failed()) {
            // Log the error or throw a custom exception
            report(new \Exception("JWST API request failed: " . $response->body()));
            return [];
        }

        return $response->json() ?? [];
    }

    /**
     * A helper to pick the most suitable image URL from the API response.
     * It prefers full-size JPGs but falls back to other formats.
     */
    public static function pickImageUrl(array $item): ?string
    {
        $location = $item['location'] ?? null;
        if (!$location) return null;

        $thumbnails = collect($item['thumbnails'] ?? []);
        $jpg = $thumbnails->firstWhere('type', 'jpg');

        return $jpg['url'] ?? $location;
    }
}
