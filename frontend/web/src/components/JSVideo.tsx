import { useEffect, useRef, useState } from "react"
import { render_webgl_canvas } from "../utils/webgl-renderer";
import { RAYMARCHING_FRAG_SRC, RAYMARCHING_VERT_SRC } from "../shaders/raymarching";

export const JSVideo = () => {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);

    useEffect(() => {

        if (!canvasRef.current) return;

        const initWebGl = async () => {
            try {
                const canvas = document.getElementById('canvas-js');
                if (canvas) {
                    console.log("Canvas found, starting webGL");
                    render_webgl_canvas("canvas-js", RAYMARCHING_VERT_SRC, RAYMARCHING_FRAG_SRC);
                } else {
                    console.error('Canvas not found in DOM');
                }
            } catch (error) {
                console.error('Error starting wegGL:', error);
            }
        };

        initWebGl();
    }, [canvasRef]);

    return (
        <section className="grid max-w-full max-h-full">
            <h1 className="text-4xl text-white">Showcase of a WASM WebGL component</h1>
            <div className="grid grid-cols-2">
                <div>
                    <h1>JS Implementation</h1>
                    <canvas id="canvas-js" className="w-xl h-auto -z-10bg-black bg-black" ref={canvasRef}></canvas>
                    <h3 className="text-3xl text-black" id="fps-counter-element">Current FPS: 0</h3>
                </div>
            </div>
        </section>
    )
}