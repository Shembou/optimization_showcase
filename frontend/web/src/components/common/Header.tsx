function Header() {
    return (
        <header className="bg-blue-600 text-white p-4">
            <div className="container mx-auto flex justify-between items-center">
                <h1 className="text-2xl">My App</h1>
                <nav className="flex space-x-6">
                    <div className="group relative">
                        <span className="cursor-pointer font-semibold">http2</span>
                        <div className="absolute hidden group-hover:block bg-white text-black rounded shadow mt-2 p-2 space-y-1">
                            <a href="/wss" className="block hover:bg-gray-100 px-2 py-1">WebSocket</a>
                            <a href="/grpcShowcase" className="block hover:bg-gray-100 px-2 py-1">gRPC</a>
                            <a href="/http2fetch" className="block hover:bg-gray-100 px-2 py-1">REST</a>
                            <a href="/graphql" className="block hover:bg-gray-100 px-2 py-1">GraphQL</a>
                        </div>
                    </div>

                    <div className="group relative">
                        <span className="cursor-pointer font-semibold">JS</span>
                        <div className="absolute hidden group-hover:block bg-white text-black rounded shadow mt-2 p-2 space-y-1">
                            <a href="/js" className="block hover:bg-gray-100 px-2 py-1">juliaCanvas</a>
                        </div>
                    </div>

                    <div className="group relative">
                        <span className="cursor-pointer font-semibold">WASM</span>
                        <div className="absolute hidden group-hover:block bg-white text-black rounded shadow mt-2 p-2 space-y-1">
                            <a href="/wasm" className="block hover:bg-gray-100 px-2 py-1">juliaCanvas</a>
                        </div>
                    </div>
                </nav>
            </div>
        </header>
    );
}

export default Header;
