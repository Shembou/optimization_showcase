import { useEffect, useRef } from "react";
import * as wasm from "rust_wasm";

function WasmVideo() {

    const canvasRef = useRef<HTMLCanvasElement | null>(null);

    useEffect(() => {
        const initWasm = async () => {
            try {
                const canvas = document.getElementById('canvas-wasm');
                if (canvas) {
                    console.log('Canvas found, starting WASM...');
                    wasm.render_accretion("canvas-wasm");
                } else {
                    console.error('Canvas not found in DOM');
                }
            } catch (error) {
                console.error('Error starting WASM:', error);
            }
        };

        initWasm();
    }, [canvasRef]);

    return (
        <section className="grid max-w-full max-h-full">
            <h1 className="text-4xl text-white">Showcase of a WASM WebGL component</h1>
            <div className="grid grid-cols-2">
                <div>
                    <h1>WASM Implementation</h1>
                    <canvas id="canvas-wasm" className="w-xl h-auto -z-10 bg-black" ref={canvasRef}></canvas>
                    <h3 className="text-3xl text-black" id="fps-counter-element">Current FPS: 0</h3>
                </div>
            </div>
        </section>
    )
}

export default WasmVideo;