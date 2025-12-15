<?php

namespace App\Jobs;

use App\Contracts\AstronomyClientInterface;
use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;
use Illuminate\Queue\InteractsWithQueue;
use Illuminate\Queue\SerializesModels;
use Illuminate\Support\Facades\Log;

class UpdateAstronomyCacheJob implements ShouldQueue
{
    use Dispatchable, InteractsWithQueue, Queueable, SerializesModels;

    /**
     * The number of times the job may be attempted.
     *
     * @var int
     */
    public int $tries = 3;

    /**
     * Create a new job instance.
     */
    public function __construct(
        private readonly float $latitude,
        private readonly float $longitude
    ) {}

    /**
     * Execute the job.
     *
     * This job will fetch astronomy events using the injected client.
     * The client itself (`RedisAstronomyCache`) is responsible for populating
     * the cache, so this job just needs to trigger the `getEvents` method.
     */
    public function handle(AstronomyClientInterface $astroClient): void
    {
        Log::info('UpdateAstronomyCacheJob started.', [
            'latitude' => $this->latitude,
            'longitude' => $this->longitude
        ]);

        try {
            // The service we resolve here is already decorated with the cache layer.
            // Calling this method will automatically fetch and cache the result.
            $astroClient->getEvents($this->latitude, $this->longitude, 7);

            Log::info('UpdateAstronomyCacheJob finished successfully.');
        } catch (\Throwable $e) {
            Log::error('UpdateAstronomyCacheJob failed.', ['error' => $e->getMessage()]);
            // Re-throw the exception to allow the queue worker to handle the failure (e.g., retry).
            throw $e;
        }
    }
}
