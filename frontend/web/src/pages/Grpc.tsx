import React, { useState } from 'react';
import { gRPC } from '../classes/gRPC';

const Grpc = () => {
    const [message, setMessage] = useState('');
    const [response, setResponse] = useState('');

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            const res = await gRPC.helloRPC(message);
            setResponse(res.getMessage());
        } catch (error) {
            setResponse('Error: ' + error);
        }
    };

    return (
        <div>
            <h1>gRPC Showcase</h1>
            <form onSubmit={handleSubmit}>
                <input
                    type="text"
                    value={message}
                    onChange={(e) => setMessage(e.target.value)}
                    placeholder="Enter your name"
                />
                <button type="submit">Send gRPC Request</button>
            </form>
            {response && (
                <div>
                    <h2>Response:</h2>
                    <p>{response}</p>
                </div>
            )}
        </div>
    );
};

export default Grpc;
