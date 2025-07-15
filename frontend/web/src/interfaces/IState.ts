export interface IState {
    gl: WebGL2RenderingContext
    vao: WebGLVertexArrayObject
    uResolution: WebGLUniformLocation
    uTime: WebGLUniformLocation
    startTime: number
    frameCount: number
    lastFpsUpdate: number
    fps: number
    frameTimes: Array<number>
}