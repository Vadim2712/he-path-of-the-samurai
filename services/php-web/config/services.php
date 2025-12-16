<?php

return [
    'astronomy' => [
        'app_id' => env('ASTRONOMY_API_ID'),
        'secret' => env('ASTRONOMY_API_SECRET'),
    ],

    'rust_iss' => [
        'base_uri' => env('RUST_ISS_BASE_URI', 'http://rust_iss:3000'),
    ],
];