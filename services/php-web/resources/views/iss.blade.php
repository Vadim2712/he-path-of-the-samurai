@extends('layouts.app')

@section('content')
<div class="container mt-4">
    <div class="card">
        <div class="card-header">
            <h4>Live ISS Position</h4>
        </div>
        <div class="card-body">
            <div id="live-iss-map" style="height: 500px;"></div>
            <div class="mt-3">
                <p><strong>Latitude:</strong> <span id="iss-lat">N/A</span></p>
                <p><strong>Longitude:</strong> <span id="iss-lon">N/A</span></p>
                <p><strong>Altitude:</strong> <span id="iss-alt">N/A</span> km</p>
                <p><strong>Velocity:</strong> <span id="iss-vel">N/A</span> km/h</p>
            </div>
        </div>
    </div>
</div>
@endsection

@push('scripts')
<script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
<script>
    document.addEventListener('DOMContentLoaded', function () {
        var map = L.map('live-iss-map').setView([0, 0], 2);
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
        }).addTo(map);

        var issIcon = L.icon({
            iconUrl: 'https://upload.wikimedia.org/wikipedia/commons/thumb/e/e3/Iss_logo.svg/100px-Iss_logo.svg.png',
            iconSize: [50, 32],
            iconAnchor: [25, 16]
        });
        var marker = L.marker([0, 0], {icon: issIcon}).addTo(map);

        function fetchIssData() {
            fetch('/api/iss/last')
                .then(response => response.json())
                .then(data => {
                    if (data && data.payload) {
                        const { latitude, longitude, altitude, velocity } = data.payload;
                        
                        document.getElementById('iss-lat').textContent = latitude.toFixed(4);
                        document.getElementById('iss-lon').textContent = longitude.toFixed(4);
                        document.getElementById('iss-alt').textContent = altitude.toFixed(2);
                        document.getElementById('iss-vel').textContent = velocity.toFixed(2);
                        
                        const latLng = [latitude, longitude];
                        marker.setLatLng(latLng);
                        map.panTo(latLng);
                    }
                })
                .catch(error => console.error('Error fetching ISS data:', error));
        }

        fetchIssData();
        setInterval(fetchIssData, 5000); // Update every 5 seconds
    });
</script>
@endpush
