// React component example
import React, { useState, useEffect, useRef } from 'react';
import { WebTransporter } from '../classes/WebTransporter';

export const Http3Fetch: React.FC = () => {
    const [isConnected, setIsConnected] = useState(false);
    const [messages, setMessages] = useState<string[]>([]);
    const [inputMessage, setInputMessage] = useState('');
    const transporterRef = useRef<WebTransporter | null>(null);

    useEffect(() => {
        const initTransport = async () => {
            try {
                const transporter = new WebTransporter();
                await transporter.initializeTransport(setIsConnected);
                transporterRef.current = transporter;

                // Start listening for incoming messages
                transporter.listenForStreams(
                    (message) => {
                        setMessages(prev => [...prev, `Server: ${message}`]);
                    },
                    (error) => {
                        console.error('Stream error:', error);
                    }
                );
            } catch (error) {
                console.error('Failed to initialize transport:', error);
                setIsConnected(false);
            }
        };

        initTransport();

        return () => {
            if (transporterRef.current) {
                transporterRef.current.close();
            }
        };
    }, []);

    const sendBidirectionalMessage = async () => {
        if (!transporterRef.current || !inputMessage.trim()) return;

        try {
            setMessages(prev => [...prev, `You: ${inputMessage}`]);
            const response = await transporterRef.current.sendMessage(inputMessage);
            setMessages(prev => [...prev, `Server: ${response}`]);
            setInputMessage('');
        } catch (error) {
            console.error('Failed to send message:', error);
        }
    };

    const sendUnidirectionalMessage = async () => {
        if (!transporterRef.current || !inputMessage.trim()) return;

        try {
            setMessages(prev => [...prev, `You (uni): ${inputMessage}`]);
            await transporterRef.current.sendUnidirectional(inputMessage);
            setInputMessage('');
        } catch (error) {
            console.error('Failed to send unidirectional message:', error);
        }
    };

    return (
        <div style={{ padding: '20px', maxWidth: '600px' }}>
            <h2>WebTransport Demo</h2>
            <div style={{ marginBottom: '20px' }}>
                Status: <span style={{ color: isConnected ? 'green' : 'red' }}>
                    {isConnected ? 'Connected' : 'Disconnected'}
                </span>
            </div>

            <div style={{ border: '1px solid #ccc', padding: '10px', height: '300px', overflowY: 'auto', marginBottom: '20px' }}>
                {messages.map((msg, index) => (
                    <div key={index} style={{ marginBottom: '5px' }}>
                        {msg}
                    </div>
                ))}
            </div>

            <div style={{ display: 'flex', gap: '10px', marginBottom: '10px' }}>
                <input
                    type="text"
                    value={inputMessage}
                    onChange={(e) => setInputMessage(e.target.value)}
                    placeholder="Enter message..."
                    style={{ flex: 1, padding: '8px' }}
                    onKeyPress={(e) => e.key === 'Enter' && sendBidirectionalMessage()}
                />
            </div>

            <div style={{ display: 'flex', gap: '10px' }}>
                <button 
                    onClick={sendBidirectionalMessage}
                    disabled={!isConnected}
                    style={{ padding: '8px 16px' }}
                >
                    Send Bidirectional
                </button>
                <button 
                    onClick={sendUnidirectionalMessage}
                    disabled={!isConnected}
                    style={{ padding: '8px 16px' }}
                >
                    Send Unidirectional
                </button>
            </div>
        </div>
    );
};