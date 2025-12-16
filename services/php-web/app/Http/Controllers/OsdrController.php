<?php

namespace App\Http\Controllers;

use App\Services\OsdrService;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Http;
use Illuminate\Support\Facades\Log;

class OsdrController extends Controller
{
    protected OsdrService $osdrService;

    public function __construct(OsdrService $osdrService)
    {
        $this->osdrService = $osdrService;
    }

    /**
     * Display the OSDR data browser page.
     * It fetches data from the rust-iss service, which in turn gets it from NASA's OSDR API.
     */
    public function index(Request $request)
    {
        $page = max(1, (int)$request->query('page', 1));
        $perPage = max(1, min(100, (int)$request->query('per_page', 15)));
        $searchQuery = trim($request->query('q', ''));

        $osdrData = $this->osdrService->getData($page, $perPage, $searchQuery);

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
            $rustServiceUrl = config('services.rust_iss.base_uri');
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
