from typing import Any, Dict, Optional
from fastapi import FastAPI, WebSocket
from fastapi.staticfiles import StaticFiles
import bson
from starlette import websockets

app = FastAPI()
app.mount("/static", StaticFiles(directory="static"), name="static")
keybd_client: Optional[WebSocket] = None
display_client: Optional[WebSocket] = None

@app.websocket("/keybd")
async def keybd_ws(ws: WebSocket):
    global keybd_client
    print("spaghet")
    await ws.accept()
    keybd_client = ws
    try:
        while True:
            data = bson.loads(await ws.receive_bytes())
            if display_client is not None:
                await display_client.send_json(data)
    except websockets.WebSocketDisconnect:
        if keybd_client is ws:
            keybd_client = None
        print("disconnected keyboard")


@app.websocket("/display")
async def display_ws(ws: WebSocket):
    global display_client
    await ws.accept()
    display_client = ws
    print("DISPLAY CONNECTED")

    try:
        while True:
            data: Dict[str, Any] = await ws.receive_json()
            print(f"Display sent {data}")
    except websockets.WebSocketDisconnect:
        if display_client is ws:
            display_client = None
        print("disconnected display")

