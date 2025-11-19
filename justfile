compile-shaders:
    mkdir -p shaders/out
    glslc shaders/shader.vert -o shaders/out/vert.spv
    glslc shaders/shader.frag -o shaders/out/frag.spv