// Complex number class
class Complex {
  constructor(public real: number, public imaginary: number) {}

  square(): Complex {
    const real = (this.real * this.real) - (this.imaginary * this.imaginary);
    const imaginary = 2.0 * this.real * this.imaginary;
    return new Complex(real, imaginary);
  }

  norm(): number {
    return (this.real * this.real) + (this.imaginary * this.imaginary);
  }

  add(other: Complex): Complex {
    return new Complex(
      this.real + other.real,
      this.imaginary + other.imaginary
    );
  }
}

// Main function to draw Julia set on canvas
export function drawJuliaCanvas(
  canvasId: string,
  width: number,
  height: number,
  real: number,
  imaginary: number
): void {
  const canvas = document.getElementById(canvasId) as HTMLCanvasElement;
  if (!canvas) {
    throw new Error(`Canvas with id '${canvasId}' not found`);
  }

  const ctx = canvas.getContext('2d');
  if (!ctx) {
    throw new Error('Could not get 2D context from canvas');
  }

  const c = new Complex(real, imaginary);
  const data = getJuliaSet(width, height, c);
  
  // Create ImageData from the pixel data
  const imageData = new ImageData(new Uint8ClampedArray(data), width, height);
  
  // Set canvas dimensions
  canvas.height = 600;
  canvas.width = 600;
  
  // Draw the image data to canvas
  ctx.putImageData(imageData, 0, 0);
}

// Generate Julia set pixel data
function getJuliaSet(width: number, height: number, c: Complex): number[] {
  const data: number[] = [];
  const paramI = 1.5;
  const paramR = 1.5;
  const scale = 0.005;

  for (let x = 0; x < width; x++) {
    for (let y = 0; y < height; y++) {
      const z = new Complex(
        y * scale - paramR,
        x * scale - paramI
      );
      
      const iterIndex = getIterIndex(z, c);
      
      // Add RGBA values      
      data.push(Math.floor(iterIndex / 4) & 0xFF); // Red
      data.push(Math.floor(iterIndex / 2) & 0xFF); // Green
      data.push(iterIndex & 0xFF);                 // Blue
      data.push(255);                              // Alpha
    }
  }

  return data;
}

// Calculate iteration index for Julia set
function getIterIndex(z: Complex, c: Complex): number {
  let iterIndex = 0;
  let current = new Complex(z.real, z.imaginary);

  while (iterIndex < 900) {
    if (current.norm() > 2.0) {
      break;
    }
    current = current.square().add(c);
    iterIndex++;
  }

  return iterIndex;
}