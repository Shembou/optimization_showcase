import { type FormEvent, useState } from "react"
import { Fetch } from "../classes/Fetch";
import { MethodTypes } from "../enums/MethodTypes";


export const Http1_1Fetch = () => {

    const iterations = 2000;

    const [didComplete, setDidComplete] = useState(false);
    const [elapsedTime, setElapsedTime] = useState<number | null>(null);

    async function handleSubmit(event: FormEvent) {
        event.preventDefault();
        const fetch = new Fetch<any>();
        const startTime = performance.now();
        const fetchPromises = Array.from({ length: iterations }, (_, i) =>
            fetch.fetchData(`?id=${i}`, MethodTypes.get, "http1_1")
        );

        try {
            const results = await Promise.all(fetchPromises);
            const dataStorage = new Set(results);
            if (dataStorage.size === iterations) {
                setDidComplete(true);
                const endTime = performance.now();
                setElapsedTime(endTime - startTime);
            }
        } catch (err) {
            console.error("Error during bulk fetch", err);
        }
    }

    return <>
        <section>
            <form onSubmit={(event) => handleSubmit(event)}>
                <button>Click me to fetch data </button>
            </form>
            {didComplete && <h1>Result: {elapsedTime?.toFixed(2)} ms</h1>}
        </section>
    </>
}