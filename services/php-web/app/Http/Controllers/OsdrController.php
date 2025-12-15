<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

class OsdrController extends Controller
{
    /**
     * Display the OSDR data browser page.
     * It fetches data from the rust-iss service, which in turn gets it from NASA's OSDR API.
     */
    public function index(Request $request)
    {
        // Simple pagination parameters from the request
        $page = max(1, (int)$request->query('page', 1));
        $perPage = max(1, min(100, (int)$request->query('per_page', 15)));

        // Search query parameter
        $searchQuery = trim($request->query('q', ''));

        // Build a cache key that includes pagination and search query
        $cacheKey = "osdr_data:page_{$page}:per_{$perPage}:q_" . md5($searchQuery);

        // Cache the response from the rust-iss service for 5 minutes
        $osdrData = Cache::remember($cacheKey, 300, function () use ($page, $perPage, $searchQuery) {
            $rustServiceUrl = getenv('RUST_BASE') ?: 'http://rust_iss:3000';
            
            $queryParams = [
                'page' => $page,
                'per_page' => $perPage,
                'q' => $searchQuery,
            ];

            try {
                $response = Http::timeout(10)->get("$rustServiceUrl/osdr", $queryParams);

                if ($response->failed()) {
                    Log::error('Failed to fetch OSDR data from rust_iss.', [
                        'status' => $response->status(),
                        'body' => $response->body(),
                    ]);
                    // Return a structured error response for the view
                    return ['error' => true, 'message' => 'Failed to load data from the service.'];
                }
                
                return $response->json();

            } catch (\Exception $e) {
                Log::critical('Could not connect to rust_iss for OSDR data.', ['error' => $e->getMessage()]);
                return ['error' => true, 'message' => 'Service is currently unavailable.'];
            }
        });

        // The view expects 'items' and 'total' keys for pagination.
        // We ensure they exist even if the API call fails.
        return view('osdr', [
            'items' => $osdrData['items'] ?? [],
            'total' => $osdrData['total'] ?? 0,
            'page' => $page,
            'perPage' => $perPage,
            'searchQuery' => $searchQuery,
            'error' => $osdrData['error'] ?? null,
            'errorMessage' => $osdrData['message'] ?? '',
        ]);
    }

    /**
     * Trigger a manual synchronization of the OSDR data.
     * This endpoint calls the rust-iss service to start a background job.
     */
    public function sync()
    {
        try {
            $rustServiceUrl = getenv('RUST_BASE') ?: 'http://rust_iss:3000';
            $response = Http::timeout(5)->post("$rustServiceUrl/osdr/sync");

            if ($response->failed()) {
                return back()->with('error', 'Failed to start OSDR sync. Service returned an error.');
            }

            return back()->with('success', 'OSDR data synchronization has been started.');

        } catch (\Exception $e) {
            Log::error('Failed to trigger OSDR sync job on rust_iss.', ['error' => $e->getMessage()]);
            return back()->with('error', 'Could not connect to the service to start the sync job.');
        }
    }
}
