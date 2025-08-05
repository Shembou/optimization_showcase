import { useEffect, useState } from "react";

export function useGraphQL<T>(query: string) {
    const [loading, setLoading] = useState(true);
    const [result, setResult] = useState<T | null>(null);

    useEffect(() => {
        const fetchGraphqlData = async () => {
            setLoading(true);
            try {
                const url = `https://http2_api.localhost/graphql?query=${encodeURIComponent(`{${query}}`)}`;
                const response = await fetch(url, {
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json',
                    }
                });

                if (!response.ok) {
                    throw new Error(`Response status: ${response.status}`);
                }

                const data = await response.json();
                setResult(data.data);
            } catch (error) {
                console.error('GraphQL Fetch Error:', (error as Error).message);
            } finally {
                setLoading(false);
            }
        };

        fetchGraphqlData();
    }, [query]);

    return { loading, result };
}
