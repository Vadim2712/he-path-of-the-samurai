<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class CmsController extends Controller
{
    /**
     * Display a generic CMS page.
     *
     * @param  string  $slug
     * @return \Illuminate\View\View
     */
    public function page(string $slug)
    {
        // In a real application, you would fetch content from a database or a flat-file CMS
        // based on the $slug.
        $title = ucfirst(str_replace('-', ' ', $slug));
        $content = "This is the content for the {$title} page. This would be dynamically loaded.";

        return view('cms.page', [
            'title' => $title,
            'content' => $content,
        ]);
    }
}
