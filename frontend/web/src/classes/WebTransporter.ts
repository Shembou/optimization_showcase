export class WebTransporter {
    private api_endpoint = "https://http_quic_api.localhost";
    private transport: WebTransport | null = null;

    async initializeTransport(setStateAction: React.Dispatch<React.SetStateAction<boolean>>) {
        try {
            // Check if WebTransport is supported
            if (!('WebTransport' in window)) {
                throw new Error('WebTransport is not supported in this browser');
            }

            console.log('Connecting to:', this.api_endpoint);
            this.transport = new WebTransport(this.api_endpoint);
            
            // Wait for connection to be ready
            await this.transport.ready;
            console.log('WebTransport connection established');
            
            setStateAction(true);
            return this.transport;
        } catch (error) {
            console.error('WebTransport connection failed:', error);
            setStateAction(false);
            throw error;
        }
    }

    // Send data via bidirectional stream and receive response
    async sendMessage(message: string): Promise<string> {
        if (!this.transport) {
            throw new Error('Transport not initialized');
        }

        try {
            // Create bidirectional stream
            const stream = await this.transport.createBidirectionalStream();
            const writer = stream.writable.getWriter();
            const reader = stream.readable.getReader();

            // Send message
            const encoder = new TextEncoder();
            await writer.write(encoder.encode(message));
            await writer.close();

            // Read response
            const decoder = new TextDecoder();
            let response = '';
            
            while (true) {
                const { done, value } = await reader.read();
                if (done) break;
                response += decoder.decode(value);
            }

            reader.releaseLock();
            return response;
        } catch (error) {
            console.error('Failed to send/receive message:', error);
            throw error;
        }
    }

    // Send data via unidirectional stream (fire and forget)
    async sendUnidirectional(message: string): Promise<void> {
        if (!this.transport) {
            throw new Error('Transport not initialized');
        }

        try {
            const stream = await this.transport.createUnidirectionalStream();
            const writer = stream.getWriter();
            const encoder = new TextEncoder();
            
            await writer.write(encoder.encode(message));
            await writer.close();
        } catch (error) {
            console.error('Failed to send unidirectional message:', error);
            throw error;
        }
    }

    // Listen for incoming bidirectional streams from server
    async listenForStreams(
        onMessage: (message: string) => void,
        onError?: (error: Error) => void
    ): Promise<void> {
        if (!this.transport) {
            throw new Error('Transport not initialized');
        }

        try {
            const reader = this.transport.incomingBidirectionalStreams.getReader();
            
            while (true) {
                const { done, value: stream } = await reader.read();
                if (done) break;

                // Handle each incoming stream
                this.handleIncomingStream(stream, onMessage, onError);
            }
        } catch (error) {
            console.error('Error listening for streams:', error);
            if (onError) onError(error as Error);
        }
    }

    private async handleIncomingStream(
        stream: WebTransportBidirectionalStream,
        onMessage: (message: string) => void,
        onError?: (error: Error) => void
    ): Promise<void> {
        try {
            const reader = stream.readable.getReader();
            const decoder = new TextDecoder();
            let message = '';

            while (true) {
                const { done, value } = await reader.read();
                if (done) break;
                message += decoder.decode(value);
            }

            reader.releaseLock();
            onMessage(message);
        } catch (error) {
            console.error('Error handling incoming stream:', error);
            if (onError) onError(error as Error);
        }
    }

    // Close the transport connection
    async close(): Promise<void> {
        if (this.transport) {
            this.transport.close();
            this.transport = null;
        }
    }
}