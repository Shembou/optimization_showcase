import './App.css';
import { Routes, Route } from 'react-router-dom'
import Header from './components/common/Header';
import Footer from './components/common/Footer';
import Home from './pages/Home';
import { JS } from './pages/JS';
import Wasm from './pages/Wasm';
import { Http2Fetch } from './pages/Http2Fetch';
import GrpcShowcase from './pages/grpcShowcase';
import { Http1_1Fetch } from './pages/Http1_1Fetch';

function App() {
  return (
    <>
      <Header />
      <main className="container mx-auto px-4 py-8">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/js" element={<JS />} />
          <Route path="/wasm" element={<Wasm />} />
          <Route path="/http2fetch" element={<Http2Fetch />} />
          <Route path="/http1_1fetch" element={<Http1_1Fetch />} />
          <Route path="/grpcShowcase" element={<GrpcShowcase />} />
        </Routes>
      </main>
      <Footer />
    </>
  );
}

export default App;
