<?php

namespace App\Http\Requests;

use Illuminate\Foundation\Http\FormRequest;

class GetAstroEventsRequest extends FormRequest
{
    /**
     * Determine if the user is authorized to make this request.
     *
     * @return bool
     */
    public function authorize()
    {
        // For now, everyone is authorized. Can be extended with auth logic.
        return true;
    }

    /**
     * Get the validation rules that apply to the request.
     *
     * @return array
     */
    public function rules()
    {
        return [
            'lat' => 'sometimes|numeric|min:-90|max:90',
            'lon' => 'sometimes|numeric|min:-180|max:180',
        ];
    }
}
