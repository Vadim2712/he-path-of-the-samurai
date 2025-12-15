@extends('layouts.app')

@section('content')
<div class="container mt-4">
    <div class="row">
        <div class="col-md-12">
            <div class="card">
                <div class="card-header">
                    <h1>{{ $title }}</h1>
                </div>
                <div class="card-body">
                    <p>{{ $content }}</p>
                </div>
            </div>
        </div>
    </div>
</div>
@endsection
