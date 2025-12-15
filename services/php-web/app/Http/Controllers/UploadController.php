<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class UploadController extends Controller
{
    /**
     * Handle a file upload.
     * This is a placeholder and does not store the file permanently.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Http\RedirectResponse
     */
    public function upload(Request $request)
    {
        $request->validate([
            'file_upload' => 'required|file|mimes:jpg,png,csv,txt,pdf|max:2048',
        ]);

        $fileName = $request->file_upload->getClientOriginalName();
        // In a real app, you would move the file to a permanent storage location:
        // $path = $request->file_upload->store('uploads');

        return back()
            ->with('success', "File '{$fileName}' uploaded successfully (but not stored).")
            ->with('file', $fileName);
    }
}
