import React from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter } from 'react-router-dom';
import App from './App';
import './output.css';

// Assert that 'root' exists
const container = document.getElementById('root');

if (!container) {
  throw new Error("Root container not found. Make sure there's a div with id='root' in your index.html");
}

const root = createRoot(container);
root.render(
  <BrowserRouter>
    <App />
  </BrowserRouter>
);
