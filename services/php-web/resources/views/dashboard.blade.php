@extends('layouts.app')

@section('content')
<div class="container-fluid mt-4">
    <div class="row">
        <!-- ISS Position -->
        <div class="col-md-4">
            <div class="card">
                <div class="card-header">
                    <h4>ISS Current Position</h4>
                </div>
                <div class="card-body">
                    @if(!empty($iss) && isset($iss['payload']['latitude']))
                        <p><strong>Latitude:</strong> {{ $iss['payload']['latitude'] }}</p>
                        <p><strong>Longitude:</strong> {{ $iss['payload']['longitude'] }}</p>
                        <p><strong>Timestamp:</strong> {{ date('Y-m-d H:i:s', $iss['payload']['timestamp']) }}</p>
                        <div id="iss-map" style="height: 250px;"></div>
                    @else
                        <div class="alert alert-warning">Could not retrieve ISS position.</div>
                    @endif
                </div>
            </div>
        </div>

        <!-- ISS Trend -->
        <div class="col-md-8">
            <div class="card">
                <div class="card-header">
                    <h4>ISS Altitude/Velocity Trend (Last Hour)</h4>
                </div>
                <div class="card-body">
                    @if(!empty($trend))
                        <canvas id="iss-trend-chart"></canvas>
                    @else
                        <div class="alert alert-info">Trend data is loading or unavailable.</div>
                    @endif
                </div>
            </div>
        </div>
    </div>

    <div class="row mt-4">
        <!-- Astronomy Events -->
        <div class="col-md-6">
            <div class="card">
                <div class="card-header">
                    <h4>Upcoming Celestial Events</h4>
                </div>
                <div class="card-body">
                    @if(!empty($astroEvents))
                        <ul class="list-group">
                            @foreach(collect($astroEvents)->take(5) as $event)
                                <li class="list-group-item">
                                    <strong>{{ $event['eventName'] }}:</strong>
                                    {{ \Carbon\Carbon::parse($event['eventDate'])->format('Y-m-d H:i') }}
                                </li>
                            @endforeach
                        </ul>
                    @else
                        <div class="alert alert-light">No upcoming astronomical events found for this location.</div>
                    @endif
                </div>
            </div>
        </div>

        <!-- JWST Gallery -->
        <div class="col-md-6">
            <div class="card">
                <div class="card-header">
                    <h4>JWST Image Gallery</h4>
                </div>
                <div class="card-body">
                    @if(!empty($jwstItems))
                        <div id="jwstCarousel" class="carousel slide" data-bs-ride="carousel">
                            <div class="carousel-inner">
                                @foreach($jwstItems as $index => $item)
                                    <div class="carousel-item {{ $index === 0 ? 'active' : '' }}">
                                        <img src="{{ $item['url'] }}" class="d-block w-100" alt="{{ $item['title'] }}" style="max-height: 400px; object-fit: cover;">
                                    </div>
                                @endforeach
                            </div>
                            <button class="carousel-control-prev" type="button" data-bs-target="#jwstCarousel" data-bs-slide="prev">
                                <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                                <span class="visually-hidden">Previous</span>
                            </button>
                            <button class="carousel-control-next" type="button" data-bs-target="#jwstCarousel" data-bs-slide="next">
                                <span class="carousel-control-next-icon" aria-hidden="true"></span>
                                <span class="visually-hidden">Next</span>
                            </button>
                        </div>
                    @else
                        <div class="alert alert-secondary">JWST images are currently unavailable.</div>
                    @endif
                </div>
            </div>
        </div>
    </div>
</div>
@endsection

@push('scripts')
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
<script>
    // ISS Map
    @if(!empty($iss) && isset($iss['payload']['latitude']))
        var lat = {{ $iss['payload']['latitude'] }};
        var lon = {{ $iss['payload']['longitude'] }};
        var map = L.map('iss-map').setView([lat, lon], 3);
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap contributors'
        }).addTo(map);
        var marker = L.marker([lat, lon]).addTo(map);
    @endif

    // ISS Trend Chart
    @if(!empty($trend))
        const trendCtx = document.getElementById('iss-trend-chart').getContext('2d');
        const trendData = @json($trend);
        
        new Chart(trendCtx, {
            type: 'line',
            data: {
                labels: trendData.map(d => new Date(d.timestamp * 1000).toLocaleTimeString()),
                datasets: [{
                    label: 'Altitude (km)',
                    data: trendData.map(d => d.altitude),
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.1
                }, {
                    label: 'Velocity (km/h)',
                    data: trendData.map(d => d.velocity),
                    borderColor: 'rgb(255, 99, 132)',
                    yAxisID: 'y1',
                    tension: 0.1
                }]
            },
            options: {
                scales: {
                    y: {
                        type: 'linear',
                        display: true,
                        position: 'left',
                        title: {
                            display: true,
                            text: 'Altitude (km)'
                        }
                    },
                    y1: {
                        type: 'linear',
                        display: true,
                        position: 'right',
                        title: {
                            display: true,
                            text: 'Velocity (km/h)'
                        },
                        grid: {
                            drawOnChartArea: false, // only want the grid lines for one axis to show up
                        },
                    }
                }
            }
        });
    @endif
</script>
@endpush
