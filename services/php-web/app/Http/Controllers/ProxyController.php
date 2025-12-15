<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Http;
use Illuminate\Http\Client\ConnectionException;
use Illuminate\Support\Facades\Log;
use Symfony\Component\HttpFoundation\StreamedResponse;

class ProxyController extends Controller
{
    /**
     * Proxy requests to another service (e.g., the Rust backend).
     *
     * @param string $path The path to proxy the request to.
     * @param Request $request The incoming request.
     * @return \Illuminate\Http\Client\Response|\Illuminate\Http\JsonResponse|StreamedResponse
     */
    public function proxy(string $path, Request $request)
    {
        // Determine the base URL of the service to proxy to.
        // Default to the rust-iss service if not specified.
        $baseUrl = env('PROXY_TARGET_BASE', 'http://rust_iss:3000');
        $fullUrl = $baseUrl . '/' . ltrim($path, '/');

        // Forward the query parameters from the original request.
        $queryParams = $request->query();

        try {
            // Make the request to the target service.
            $response = Http::withOptions(['stream' => true])
                ->withHeaders($request->headers->all())
                ->timeout(30) // Set a reasonable timeout
                ->get($fullUrl, $queryParams);

            // To handle large responses or streaming data, we can use a StreamedResponse.
            if ($response->successful()) {
                return new StreamedResponse(function () use ($response) {
                    $body = $response->toPsrResponse()->getBody();
                    while (!$body->eof()) {
                        echo $body->read(1024);
                        flush();
                    }
                }, 200, $response->headers());
            }

            // For failed requests, return a JSON error response.
            return response()->json([
                'ok' => false,
                'error' => [
                    'code' => 'proxy_error',
                    'message' => 'The target service returned an error.',
                    'status' => $response->status(),
                ]
            ], $response->status());

        } catch (ConnectionException $e) {
            Log::error("Proxy connection failed for path: {$path}", ['error' => $e->getMessage()]);
            return response()->json([
                'ok' => false,
                'error' => [
                    'code' => 'proxy_connection_failed',
                    'message' => 'Could not connect to the target service.'
                ]
            ], 502); // 502 Bad Gateway
        }
    }
}
