import React from "react";

function Header() {
    return (
        <header className="bg-blue-600 text-white p-4">
            <div className="container mx-auto">
                <h1 className="text-2xl">My App</h1>
                <nav>
                    <a href="/wasm" className="px-4">Wasm</a>
                    <a href="/js" className="px-4">Js</a>
                </nav>
            </div>
        </header>)
}

export default Header;