<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Cache;
use App\Support\JwstHelper;
use App\Contracts\AstronomyClientInterface;
use Illuminate\Support\Facades\Log;

class DashboardController extends Controller
{
    /**
     * The dashboard controller constructor.
     * Injects the Astronomy client to fetch celestial events.
     */
    public function __construct(
        private readonly AstronomyClientInterface $astronomyService
    ) {}

    /**
     * Gathers all data for and displays the main dashboard view.
     */
    public function index()
    {
        // --- Fetch ISS Data ---
        $issData = Cache::remember('dashboard:iss_position', 10, function () {
            $rustServiceUrl = getenv('RUST_BASE') ?: 'http://rust_iss:3000';
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
            $rustServiceUrl = getenv('RUST_BASE') ?: 'http://rust_iss:3000';
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
        $jwstImages = Cache::remember('dashboard:jwst_gallery', 300, function () {
            try {
                $jwstHelper = new JwstHelper();
                $apiResponse = $jwstHelper->get('all/type/jpg', ['page' => 1, 'perPage' => 9]);
                $imageList = $apiResponse['body'] ?? ($apiResponse['data'] ?? []);
                
                $formattedImages = [];
                foreach ($imageList as $item) {
                    $imageUrl = JwstHelper::pickImageUrl($item);
                    if ($imageUrl) {
                        $formattedImages[] = [
                            'url' => $imageUrl,
                            'title' => $item['details']['mission'] ?? 'JWST Image',
                            'id' => $item['id'] ?? uniqid(),
                        ];
                    }
                }
                return $formattedImages;
            } catch (\Exception $e) {
                Log::error('Dashboard failed to fetch JWST images.', ['error' => $e->getMessage()]);
                return [];
            }
        });

        return view('dashboard', [
            'iss' => $issData,
            'trend' => $issTrend,
            'astroEvents' => $astroEvents,
            'jwstItems' => $jwstImages,
        ]);
    }

    /**
     * This API endpoint is kept for backward compatibility or direct use by some clients.
     * It proxies requests to the JWST helper.
     */
    public function jwstFeed(Request $r)
    {
        $source = $r->query('source', 'jpg');
        $suffix = trim((string)$r->query('suffix', ''));
        $programId = trim((string)$r->query('program', ''));
        $instrumentFilter = strtoupper(trim((string)$r->query('instrument', '')));
        $page = max(1, (int)$r->query('page', 1));
        $perPage = max(1, min(60, (int)$r->query('perPage', 24)));

        $jwstHelper = new JwstHelper();

        $apiPath = 'all/type/jpg';
        if ($source === 'suffix' && $suffix !== '') $apiPath = 'all/suffix/' . ltrim($suffix, '/');
        if ($source === 'program' && $programId !== '') $apiPath = 'program/id/' . rawurlencode($programId);

        $response = $jwstHelper->get($apiPath, ['page' => $page, 'perPage' => $perPage]);
        $list = $response['body'] ?? ($response['data'] ?? (is_array($response) ? $response : []));

        $items = [];
        foreach ($list as $item) {
            if (!is_array($item)) continue;

            $imageUrl = JwstHelper::pickImageUrl($item);
            if (!$imageUrl) continue;

            $instrumentList = array_map('strtoupper', array_column($item['details']['instruments'] ?? [], 'instrument'));
            if ($instrumentFilter && !in_array($instrumentFilter, $instrumentList, true)) continue;

            $items[] = [
                'url'      => $imageUrl,
                'obs'      => (string)($item['observation_id'] ?? $item['observationId'] ?? ''),
                'program'  => (string)($item['program'] ?? ''),
                'suffix'   => (string)($item['details']['suffix'] ?? $item['suffix'] ?? ''),
                'inst'     => $instrumentList,
                'caption'  => 'OBS: ' . ($item['observation_id'] ?? $item['id'] ?? 'N/A'),
                'link'     => $item['location'] ?? $imageUrl,
            ];
            if (count($items) >= $perPage) break;
        }

        return response()->json([
            'source' => $apiPath,
            'count'  => count($items),
            'items'  => $items,
        ]);
    }
}
