import { type MethodType } from "../enums/MethodTypes";

export async function fetchData<T>(url: string, method: MethodType, body?: any): Promise<T> {
    const headers = new Headers();
    headers.append("Content-Type", "application/json");

    const response = body ? await fetch(`/api/${url}`, {
        method,
        headers,
        body,
    }) : await fetch(`/api/${url}`, {
        method,
        headers
    });

    if (!response.ok) {
        throw new Error(`HTTP error: ${response.status}`);
    }

    console.log(response);

    const result = await response.json();
    return result as T;
}
