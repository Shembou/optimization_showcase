import { type FormEvent } from "react";

function WebSocketPage() {
    async function startTransport(e: FormEvent) {
        e.preventDefault();
        try {
            const ws = new WebSocket("https://http2_api.localhost/ws");
            console.log("initializing websocket transport");
            ws.onmessage = (msg) => console.log(msg.data);
            ws.onopen = () => ws.send("Hello Server!");
        }
        catch (e) {
            console.log(`Error while initializing tranport. ${e}`)
        }
    }


    return (
        <>
            <section className="grid">
                <form onSubmit={(e) => startTransport(e)}>
                    <button>Click me to start the transport</button>
                </form>
            </section>
        </>
    )
}

export default WebSocketPage;