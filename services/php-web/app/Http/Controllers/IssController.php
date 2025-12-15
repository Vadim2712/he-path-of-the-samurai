<?php

namespace App\Http\Controllers;

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
        // All data fetching and map logic is handled by JavaScript on the front-end,
        // which calls the /api/iss/last and /api/iss/trend endpoints (proxied to rust-iss).
        return view('iss');
    }
}
