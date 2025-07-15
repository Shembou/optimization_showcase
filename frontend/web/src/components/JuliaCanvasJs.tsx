import { useEffect, useRef, useState } from "react";
import * as wasm from "rust_wasm";
import { drawJuliaCanvas } from "../utils/canvas-utils";

const JuliaCanvasJs = () => {

    const [timestamp,setTimestamp] = useState<number | null>(null);

    const canvasRef = useRef<HTMLCanvasElement | null>(null);

    useEffect(() => {
        const initWasm = () => {
            try {
                const canvas = document.getElementById('canvas-julia');
                if (canvas) {
                    var start_time = performance.now();
                    drawJuliaCanvas("canvas-julia",1000,1000,-0.15,0.65);
                    var end_time = performance.now();
                    setTimestamp(end_time - start_time);
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
            <h1 className="text-4xl text-white">Javascript native canvas operation</h1>
            <div className="grid grid-cols-2">
                <div>
                    <h1>Julia set</h1>
                    <canvas id="canvas-julia" className="-z-10 bg-black" ref={canvasRef}></canvas>
                    {timestamp && <h3 className="text-3xl text-black">JS implementation Julia set execution time: {timestamp} ms</h3>}
                </div>
            </div>
        </section>
    )
}

export default JuliaCanvasJs;