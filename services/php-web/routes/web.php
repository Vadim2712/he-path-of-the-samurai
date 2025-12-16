<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\DashboardController;
use App\Http\Controllers\IssController;
use App\Http\Controllers\OsdrController;
use App\Http\Controllers\CmsController;
use App\Http\Controllers\ProxyController;
use App\Http\Controllers\AstroController;

// Main dashboard
Route::get('/', [DashboardController::class, 'index']);
Route::get('/dashboard', [DashboardController::class, 'index']);

// ISS and OSDR pages
Route::get('/iss', [IssController::class, 'index']);
Route::get('/osdr', [OsdrController::class, 'index']);
Route::post('/osdr/sync', [OsdrController::class, 'sync']);

// Generic CMS page
Route::get('/page/{slug}', [CmsController::class, 'page']);

// API proxies
Route::prefix('api')->group(function () {
    // ISS data endpoints with caching
    Route::get('/iss/last', [IssController::class, 'last']);
    Route::get('/iss/trend', [IssController::class, 'trend']);
    
    // Proxy for JWST feed
    Route::get('/jwst', [DashboardController::class, 'jwstFeed']);

    // Endpoint for astronomy events with async job dispatch
    Route::get('/astro/events', [AstroController::class, 'events']);
});