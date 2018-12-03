#include "imgui/imgui.h"
#include <emscripten/bind.h>

using namespace emscripten;

EMSCRIPTEN_BINDINGS(imgui) {
    function("SliderFloat", &ImGui::SliderFloat, allow_raw_pointers());
}