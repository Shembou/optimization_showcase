import { IState } from "../interfaces/IState";

export function compileShader(gl: WebGL2RenderingContext, type: number, source: string) {
    const shader = gl.createShader(type);
    if (shader == null) {
        return;
    }
    gl.shaderSource(shader, source);
    gl.compileShader(shader);

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        const error = gl.getShaderInfoLog(shader);
        gl.deleteShader(shader);
        throw new Error(`Shader compilation error: ${error}`);
    }

    return shader;
}

export function linkProgram(gl: WebGL2RenderingContext, vertexShader: WebGLShader | undefined, fragmentShader: WebGLShader | undefined) {
    if (vertexShader == undefined || fragmentShader == undefined) {
        return;
    }
    const program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        const error = gl.getProgramInfoLog(program);
        gl.deleteProgram(program);
        throw new Error(`Program linking error: ${error}`);
    }

    return program;
}

export function render_webgl(state: IState, height: number, width: number, timestamp: number = 0) {
    // FRAME TRACKING
    state.frameCount++;

    // Store frame time for averaging
    state.frameTimes.push(timestamp);
    if (state.frameTimes.length > 60) {
        state.frameTimes.shift();
    }

    // Calculate FPS every second
    if (timestamp - state.lastFpsUpdate >= 1000.0) {
        state.fps = state.frameCount / ((timestamp - state.startTime) / 1000.0);
        state.lastFpsUpdate = timestamp;

        // Log frame statistics
        console.log(
            `JS - Frame: ${state.frameCount}`,
            `FPS: ${state.fps.toFixed(1)}`,
            `Time: ${((timestamp - state.startTime) / 1000.0).toFixed(2)}s`
        );

        // Update FPS display if element exists
        const fpsElement = document.getElementById("fps-counter-element");
        if (fpsElement) {
            fpsElement.innerHTML = `JS FPS: ${state.fps.toFixed(1)}`;
        }
    }

    // RENDERING
    // Time uniform (seconds since start)
    const t = (timestamp - state.startTime) / 1000.0;
    state.gl.uniform1f(state.uTime, t);

    // Resolution uniform (updated in case the canvas was resized)
    state.gl.uniform3f(state.uResolution, height, width, 0.0);

    // Rendering
    state.gl.viewport(0, 0, height, width);
    state.gl.clearColor(0.0, 0.0, 0.0, 1.0); // Changed to black background
    state.gl.clear(state.gl.COLOR_BUFFER_BIT);
    state.gl.bindVertexArray(state.vao);
    state.gl.drawArrays(state.gl.TRIANGLES, 0, 6);

    // Queue next frame
    requestAnimationFrame((timestamp) => render_webgl(state, height, width, timestamp));
}