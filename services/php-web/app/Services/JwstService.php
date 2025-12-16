<?php

namespace App\Services;

use App\Support\JwstHelper;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Cache;
use Illuminate\Support\Facades\Log;

class JwstService
{
    private JwstHelper $jwstHelper;

    public function __construct()
    {
        $this->jwstHelper = new JwstHelper();
    }

    /**
     * Get a formatted list of JWST images for the dashboard, with caching.
     *
     * @return array
     */
    public function getDashboardImages(): array
    {
        return Cache::remember('dashboard:jwst_gallery', 300, function () {
            try {
                $apiResponse = $this->jwstHelper->get('all/type/jpg', ['page' => 1, 'perPage' => 9]);
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
    }

    /**
     * Get a filterable feed of JWST images.
     *
     * @param Request $request
     * @return array
     */
    public function getFeed(Request $request): array
    {
        $source = $request->query('source', 'jpg');
        $suffix = trim((string)$request->query('suffix', ''));
        $programId = trim((string)$request->query('program', ''));
        $instrumentFilter = strtoupper(trim((string)$request->query('instrument', '')));
        $page = max(1, (int)$request->query('page', 1));
        $perPage = max(1, min(60, (int)$request->query('perPage', 24)));

        $apiPath = 'all/type/jpg';
        if ($source === 'suffix' && $suffix !== '') $apiPath = 'all/suffix/' . ltrim($suffix, '/');
        if ($source === 'program' && $programId !== '') $apiPath = 'program/id/' . rawurlencode($programId);

        $response = $this->jwstHelper->get($apiPath, ['page' => $page, 'perPage' => $perPage]);
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

        return [
            'source' => $apiPath,
            'count'  => count($items),
            'items'  => $items,
        ];
    }
}
