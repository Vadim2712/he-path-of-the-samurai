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
    public $tries = 3;

    /**
     * Create a new job instance.
     *
     * @return void
     */
    public function __construct(
        private float $lat,
        private float $lon
    ) {}

    /**
     * Execute the job.
     *
     * @return void
     */
    public function handle(AstronomyClientInterface $client)
    {
        Log::info('UpdateAstronomyCacheJob started.', ['lat' => $this->lat, 'lon' => $this->lon]);
        try {
            // This will automatically cache the result due to the decorator
            $client->getEvents($this->lat, $this->lon, 7);
            Log::info('UpdateAstronomyCacheJob finished successfully.');
        } catch (\Throwable $e) {
            Log::error('UpdateAstronomyCacheJob failed.', ['error' => $e->getMessage()]);
            // The job will be retried automatically based on the $tries property.
            throw $e;
        }
    }
}
