<?php

namespace App\Http\Controllers;

use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Http;
use Illuminate\Http\Request;

class IssController extends Controller
{
    /**
     * Display the ISS tracking page.
     *
     * @return \Illuminate\View\View
     */
    public function index()
    {
        // This controller simply returns the view.
        // The data fetching is now handled by dedicated API endpoints in this controller.
        return view('iss');
    }

    /**
     * Get the last known ISS position, with caching.
     *
     * @return \Illuminate\Http\JsonResponse
     */
    public function last()
    {
        $data = Cache::remember('iss_last', 10, function () {
            $response = Http::get('http://rust_iss:3000/last');
            return $response->json();
        });

        return response()->json($data);
    }

    /**
     * Get the ISS movement trend, with caching.
     *
     * @return \Illuminate\Http\JsonResponse
     */
    public function trend()
    {
        $data = Cache::remember('iss_trend', 10, function () {
            $response = Http::get('http://rust_iss:3000/iss/trend');
            return $response->json();
        });

        return response()->json($data);
    }
}
