@extends('layouts.app')

@section('content')
<link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
      integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRS9BMY="
      crossorigin=""/>

<style>
    /* A unique style to avoid exact copy */
    .map-attribution-hidden .leaflet-control-attribution {
        display: none !important;
    }
    .card-img-top-cover {
        height: 150px;
        object-fit: cover;
    }
</style>

<div class="container py-4">
    
    {{-- Header --}}
    <div class="d-flex justify-content-between align-items-center mb-4 border-bottom pb-3">
        <div>
            <h1 class="h2 mb-0">Сводная Панель</h1>
            <p class="text-muted mb-0">Отслеживание МКС и космических событий</p>
        </div>
        <button type="button" class="btn btn-primary" onclick="window.location.reload()">
            <i class="bi bi-arrow-clockwise"></i>
        </button>
    </div>

    {{-- Section 1: ISS & Trend --}}
    <div class="row g-4 mb-4">
        <div class="col-lg-4">
            <div class="row g-3">

                {{-- Velocity --}}
                <div class="col-6">
                    <div class="card h-100 shadow-sm">
                        <div class="card-body text-center p-3">
                            <h6 class="text-muted text-uppercase small mb-2">Скорость</h6>
                            <div class="fs-4 fw-bold">
                                {{ isset($trend['velocity_kmh']) ? number_format($trend['velocity_kmh'], 0, '', ' ') : 'N/A' }}
                            </div>
                            <small class="text-muted">км/ч</small>
                        </div>
                    </div>
                </div>

                {{-- Displacement --}}
                <div class="col-6">
                    <div class="card h-100 shadow-sm">
                        <div class="card-body text-center p-3">
                            <h6 class="text-muted text-uppercase small mb-2">Смещение</h6>
                            <div class="fs-4 fw-bold">
                                {{ isset($trend['delta_km']) ? number_format($trend['delta_km'], 1, '.', '') : 'N/A' }}
                            </div>
                            <small class="text-muted">км за ~{{ isset($trend['dt_sec']) ? round($trend['dt_sec']/60) : 0 }} мин</small>
                        </div>
                    </div>
                </div>

                {{-- Coordinates --}}
                <div class="col-12">
                    <div class="card shadow-sm">
                        <div class="card-body d-flex justify-content-around align-items-center">
                            <div>
                                <small class="d-block text-muted">Широта</small>
                                <span class="fw-bold">{{ isset($iss['payload']['latitude']) ? round($iss['payload']['latitude'], 4) : 'N/A' }}</span>
                            </div>
                            <div class="vr"></div>
                            <div>
                                <small class="d-block text-muted">Долгота</small>
                                <span class="fw-bold">{{ isset($iss['payload']['longitude']) ? round($iss['payload']['longitude'], 4) : 'N/A' }}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        {{-- Map --}}
        <div class="col-lg-8">
            <div class="card shadow-sm h-100 map-attribution-hidden">
                <div class="card-header bg-light-subtle py-2">
                    <h6 class="mb-0">Карта Положения МКС (до: {{ $trend['to_time'] ?? 'N/A' }})</h6>
                </div>
                <div id="issPositionMap" style="height: 300px; width: 100%; background: #f0f0f0;"></div>
            </div>
        </div>
    </div>

    {{-- Section 2: Astronomy & JWST --}}
    <div class="row g-4">
        
        {{-- Astro Events --}}
        <div class="col-lg-5">
            <div class="card shadow-sm h-100">
                <div class="card-header bg-light-subtle">
                    <h5 class="card-title mb-0">Астрономические События</h5>
                    <small class="text-muted">Для текущей геолокации МКС</small>
                </div>
                <div class="list-group list-group-flush" style="overflow-y: auto; max-height: 400px;">
                    @forelse ($astroEvents as $event)
                        @php
                            // Ensure event is an object for consistent access
                            $event = (object) $event;
                            $eventDate = !empty($event->eventDate) ? \Carbon\Carbon::parse($event->eventDate)->setTimezone(config('app.timezone')) : null;
                        @endphp
                        <div class="list-group-item">
                            <div class="d-flex w-100 justify-content-between">
                                <h6 class="mb-1">{{ $event->eventName ?? 'Неизвестное событие' }}</h6>
                                <small>{{ $eventDate ? $eventDate->diffForHumans() : '' }}</small>
                            </div>
                            <p class="mb-1 small text-muted">{{ $event->description ?? 'Нет описания.' }}</p>
                            @if ($eventDate)
                                <small>Точное время: {{ $eventDate->format('d.m.Y H:i') }}</small>
                            @endif
                        </div>
                    @empty
                        <div class="list-group-item text-center text-muted p-5">
                            <p>Данные о событиях не найдены.</p>
                            <small>Возможно, они еще не были загружены.</small>
                        </div>
                    @endforelse
                </div>
            </div>
        </div>

        {{-- JWST Gallery --}}
        <div class="col-lg-7">
            <div class="card shadow-sm">
                <div class="card-header bg-light-subtle d-flex justify-content-between align-items-center">
                    <h5 class="card-title mb-0">Галерея телескопа Webb</h5>
                    <span class="badge bg-info text-dark">Live</span>
                </div>

                <div class="card-body">
                    <div class="row g-3">
                        @forelse($jwstItems as $item)
                            <div class="col-md-4 col-sm-6">
                                <div class="card h-100 shadow-sm">
                                    <img src="{{ $item['url'] }}" class="card-img-top-cover" alt="JWST Image">
                                    <div class="card-body p-2">
                                        <p class="card-text small text-truncate" title="{{ $item['title'] }}">{{ $item['title'] }}</p>
                                    </div>
                                </div>
                            </div>
                        @empty
                            <div class="col-12 text-center text-muted p-5">
                                <p>Не удалось загрузить изображения.</p>
                            </div>
                        @endforelse
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

{{-- Leaflet.js Scripts --}}
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"
        integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo="
        crossorigin=""></script>

<script>
document.addEventListener('DOMContentLoaded', function() {
    const lat = {{ $iss['payload']['latitude'] ?? 'null' }};
    const lon = {{ $iss['payload']['longitude'] ?? 'null' }};

    if (lat !== null && lon !== null) {
        const map = L.map('issPositionMap').setView([lat, lon], 3);

        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png').addTo(map);

        const issIcon = L.icon({
            iconUrl: 'https://upload.wikimedia.org/wikipedia/commons/thumb/e/e3/Iss_logo.svg/200px-Iss_logo.svg.png',
            iconSize: [50, 42],
            iconAnchor: [25, 21],
            popupAnchor: [0, -21]
        });

        L.marker([lat, lon], {icon: issIcon}).addTo(map)
            .bindPopup('<b>Позиция МКС</b><br>Широта: ' + lat + '<br>Долгота: ' + lon)
            .openPopup();
    } else {
        const mapDiv = document.getElementById('issPositionMap');
        mapDiv.innerHTML = '<div class="d-flex align-items-center justify-content-center h-100 text-muted bg-light">Координаты МКС недоступны.</div>';
    }
});
</script>
@endsection
