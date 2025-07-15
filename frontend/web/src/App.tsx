import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Header from './components/common/Header';
import Footer from './components/common/Footer'
import { JS } from './pages/JS';
import Home from './pages/Home';
import Wasm from './pages/Wasm';

function App() {
    return (
        <>
            <Header />
            <main className="container mx-auto px-4 py-8">
                <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/js" element={<JS />} />
                    <Route path="/wasm" element={<Wasm />} />
                </Routes>
            </main>
            <Footer />
        </>
    );
}

export default App;