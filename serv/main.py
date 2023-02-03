from fastapi import FastAPI, WebSocket

app = FastAPI()


@app.websocket("/message-io-default")
async def websocket_endpoint(websocket: WebSocket):
    print("spaghet")
    await websocket.accept()
    while True:
        data = await websocket.receive_bytes()
        await websocket.send_bytes(bytes(f"fk u {data.decode('ascii')}", 'ascii'))
        print(f"received {data.decode('ascii')}")

