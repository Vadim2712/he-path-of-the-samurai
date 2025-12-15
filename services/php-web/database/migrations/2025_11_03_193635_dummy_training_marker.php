<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        // This is a dummy file to make sure the migrations directory exists.
        // In a real project, this would contain schema definitions.
        if (!Schema::hasTable('dummy_training_marker')) {
            Schema::create('dummy_training_marker', function (Blueprint $table) {
                $table->id();
                $table->string('marker')->default('This table is for training purposes.');
                $table->timestamps();
            });
        }
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('dummy_training_marker');
    }
};
