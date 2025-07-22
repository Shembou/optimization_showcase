import { HelloRequest } from "../generated/greeter_pb";
import { GreeterClient } from "../generated/GreeterServiceClientPb";

export class gRPC {
    private static client: GreeterClient = this.setupClient();

    public static async helloRPC(name: string) {
        try {
            console.log('Making gRPC call...');

            const req = new HelloRequest();
            req.setName(name);

            // Use promise-based approach for better error handling
            const response = await new Promise((resolve, reject) => {
                this.client.helloRPC(req, {}, (err, res) => {
                    if (err) {
                        console.error('gRPC error:', err);
                        reject(err);
                    } else {
                        console.log('Response:', res?.getMessage());
                        resolve(res);
                    }
                });
            });

            return response;
        } catch (error) {
            console.error('Failed to make gRPC call:', error);
            throw error;
        }
    }

    

    private static setupClient() {
        const client = new GreeterClient('https://localhost/grpc');

        // Add some debugging
        console.log('gRPC client created with base URL: /grpc');

        return client;
    }
}