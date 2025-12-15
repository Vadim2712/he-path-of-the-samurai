<?php

namespace App\Http\Requests;

use Illuminate\Foundation\Http\FormRequest;

class GetAstroEventsRequest extends FormRequest
{
    /**
     * Determine if the user is authorized to make this request.
     * For this public endpoint, we can return true.
     *
     * @return bool
     */
    public function authorize(): bool
    {
        return true;
    }

    /**
     * Get the validation rules that apply to the request.
     *
     * @return array<string, \Illuminate\Contracts\Validation\ValidationRule|array<mixed>|string>
     */
    public function rules(): array
    {
        return [
            'lat' => ['required', 'numeric', 'between:-90,90'],
            'lon' => ['required', 'numeric', 'between:-180,180'],
        ];
    }
}
