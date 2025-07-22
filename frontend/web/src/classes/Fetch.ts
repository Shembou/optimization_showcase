import type { MethodType } from "../enums/MethodTypes";

const FetchProtocol = {
    http1_1: "https://http1_1_api.localhost",
    http2: "https://http2_api.localhost"
} as const

// Define as union of string literals

export class Fetch<T = any> {
    public async fetchData(
        url: string,
        method: MethodType,
        fetchType: keyof typeof FetchProtocol,
        body?: any
    ): Promise<T> {
        const headers = new Headers({
            "Content-Type": "application/json",
        });
        console.log(FetchProtocol[fetchType])

        const response = await fetch(`${FetchProtocol[fetchType]}/api/${url}`, {
            method,
            headers,
            ...(body && { body: JSON.stringify(body) }),
        });

        if (!response.ok) {
            throw new Error(`HTTP error: ${response.status}`);
        }

        return response.json() as Promise<T>;
    }
}