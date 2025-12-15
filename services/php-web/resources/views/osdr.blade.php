@extends('layouts.app')

@section('content')
<div class="container mt-4">
    <div class="card">
        <div class="card-header">
            <h4>OSDR Data Browser</h4>
        </div>
        <div class="card-body">
            <!-- Search and Sync -->
            <div class="row mb-3">
                <div class="col-md-8">
                    <form action="{{ url('/osdr') }}" method="GET">
                        <div class="input-group">
                            <input type="text" name="q" class="form-control" placeholder="Search datasets..." value="{{ $searchQuery ?? '' }}">
                            <button class="btn btn-primary" type="submit">Search</button>
                        </div>
                    </form>
                </div>
                <div class="col-md-4 text-end">
                    <form action="{{ url('/osdr/sync') }}" method="POST">
                        @csrf
                        <button type="submit" class="btn btn-secondary">Sync OSDR Data</button>
                    </form>
                </div>
            </div>

            @if(session('success'))
                <div class="alert alert-success">{{ session('success') }}</div>
            @endif
            @if(session('error'))
                <div class="alert alert-danger">{{ session('error') }}</div>
            @endif

            <!-- Data Table -->
            @if(!empty($items))
                <table class="table table-bordered table-striped">
                    <thead>
                        <tr>
                            <th>Accession</th>
                            <th>Title</th>
                            <th>Study Type</th>
                            <th>Year</th>
                        </tr>
                    </thead>
                    <tbody>
                        @foreach($items as $item)
                            <tr>
                                <td>{{ $item['accession'] ?? 'N/A' }}</td>
                                <td>{{ Str::limit($item['title'] ?? 'No Title', 100) }}</td>
                                <td>{{ $item['study_type'] ?? 'N/A' }}</td>
                                <td>{{ $item['year'] ?? 'N/A' }}</td>
                            </tr>
                        @endforeach
                    </tbody>
                </table>

                <!-- Pagination -->
                @if($total > $perPage)
                    <nav>
                        <ul class="pagination">
                            @php
                                $totalPages = ceil($total / $perPage);
                            @endphp
                            @for($i = 1; $i <= $totalPages; $i++)
                                <li class="page-item {{ $i == $page ? 'active' : '' }}">
                                    <a class="page-link" href="{{ url('/osdr') }}?page={{ $i }}&per_page={{ $perPage }}&q={{ urlencode($searchQuery) }}">{{ $i }}</a>
                                </li>
                            @endfor
                        </ul>
                    </nav>
                @endif
            @else
                <div class="alert alert-info">No data available. Try a different search or sync the data.</div>
            @endif
        </div>
    </div>
</div>
@endsection
