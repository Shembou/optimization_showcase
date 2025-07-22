import { compileShader, linkProgram, render_webgl } from "./webgl-utils";

export function render_webgl_canvas(canvas_id: string, vert_src: string, frag_src: string) {
    try {
        console.log("Starting JavaScript WebGL implementation...");

        // DOM & GL INITIALISATION
        const canvas = document.getElementById(canvas_id) as HTMLCanvasElement;
        if (!canvas) {
            console.error("Canvas with id 'canvas-js' not found!");
            return;
        }

        const gl = canvas.getContext("webgl2");
        if (!gl) {
            throw new Error("WebGL2 not available");
        }

        console.log("WebGL2 context created successfully");

        // SHADERS & PROGRAM
        const vertShader = compileShader(gl, gl.VERTEX_SHADER, vert_src);
        const fragShader = compileShader(gl, gl.FRAGMENT_SHADER, frag_src);
        const program = linkProgram(gl, vertShader, fragShader);
        if (program == undefined) {
            return;
        }
        gl.useProgram(program);

        console.log("Shaders compiled and linked successfully");

        // GEOMETRY: FULL-SCREEN TRIANGLE STRIP
        const vertices = new Float32Array([
            -1.0, -1.0, // left-bottom
            1.0, -1.0, // right-bottom
            -1.0, 1.0, // left-top
            -1.0, 1.0, // 
            1.0, -1.0, // 
            1.0, 1.0, // right-top
        ]);

        const vao = gl.createVertexArray();
        gl.bindVertexArray(vao);

        const vbo = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, vbo);
        gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);

        const posLoc = gl.getAttribLocation(program, "a_position");
        gl.enableVertexAttribArray(posLoc);
        gl.vertexAttribPointer(posLoc, 2, gl.FLOAT, false, 0, 0);

        // UNIFORMS
        const uResolution = gl.getUniformLocation(program, "iResolution");
        const uTime = gl.getUniformLocation(program, "iTime");

        if (!uResolution || !uTime) {
            throw new Error("Required uniforms not found");
        }

        console.log("Geometry and uniforms set up successfully");

        // STATE OBJECT WITH FRAME TRACKING
        const state = {
            gl,
            vao,
            uResolution,
            uTime,
            startTime: performance.now(),
            frameCount: 0,
            lastFpsUpdate: performance.now(),
            fps: 0,
            frameTimes: []
        };

        // Get canvas dimensions
        const width = 1000;
        const height = 1000;
        canvas.width = width;
        canvas.height = height;

        console.log(`Canvas dimensions: ${width}x${height}`);
        // Start the render loop
        console.log("Starting render loop...");
        requestAnimationFrame((timestamp) => render_webgl(state, width, height, timestamp));

    } catch (error) {
        console.error("WebGL initialization error:", error);
        throw error;
    }
}