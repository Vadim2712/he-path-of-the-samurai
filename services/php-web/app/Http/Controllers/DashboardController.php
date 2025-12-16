<?php

namespace App\Http\Controllers;

use App\Contracts\AstronomyClientInterface;
use App\Services\JwstService;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

class DashboardController extends Controller
{
    /**
     * The dashboard controller constructor.
     */
    public function __construct(
        private readonly AstronomyClientInterface $astronomyService,
        private readonly JwstService $jwstService
    ) {}

    /**
     * Gathers all data for and displays the main dashboard view.
     */
    public function index()
    {
        // --- Fetch ISS Data ---
        $issData = Cache::remember('dashboard:iss_position', 10, function () {
            $rustServiceUrl = config('services.rust_iss.base_uri');
            try {
                $response = Http::timeout(3)->get("$rustServiceUrl/last");
                return $response->failed() ? [] : array_change_key_case($response->json(), CASE_LOWER);
            } catch (\Exception $e) {
                Log::warning('Failed to fetch ISS position from rust_iss.', ['error' => $e->getMessage()]);
                return [];
            }
        });

        // --- Fetch ISS Trend Data ---
        $issTrend = Cache::remember('dashboard:iss_trend', 35, function () {
            $rustServiceUrl = config('services.rust_iss.base_uri');
            try {
                $response = Http::timeout(3)->get("$rustServiceUrl/iss/trend");
                return $response->failed() ? [] : $response->json();
            } catch (\Exception $e) {
                Log::warning('Failed to fetch ISS trend from rust_iss.', ['error' => $e->getMessage()]);
                return [];
            }
        });

        // --- Fetch Astronomy Events ---
        $latitude = $issData['payload']['latitude'] ?? 55.75;
        $longitude = $issData['payload']['longitude'] ?? 37.61;
        $astroCacheKey = "dashboard:astro_events:" . round($latitude, 2) . ":" . round($longitude, 2);

        // This data is cached for a long time as it's expensive to fetch
        $astroEvents = Cache::remember($astroCacheKey, 3600, function () use ($latitude, $longitude) {
            try {
                // Use the injected service (which has its own caching layer)
                return $this->astronomyService->getEvents($latitude, $longitude, 365)->all();
            } catch (\Exception $e) {
                Log::error('Dashboard failed to fetch astronomy events.', ['error' => $e->getMessage()]);
                return [];
            }
        });

        // --- Fetch JWST Gallery Images ---
        $jwstImages = $this->jwstService->getDashboardImages();

        return view('dashboard', [
            'iss' => $issData,
            'trend' => $issTrend,
            'astroEvents' => $astroEvents,
            'jwstItems' => $jwstImages,
        ]);
    }

    /**
     * Provides a filterable feed of JWST images.
     */
    public function jwstFeed(Request $request)
    {
        $data = $this->jwstService->getFeed($request);
        return response()->json($data);
    }
}
