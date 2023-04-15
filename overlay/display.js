const socket = new WebSocket('ws://127.0.0.1:4444/');
let keysQueue = [],
    keysTimeout,
    transitionTimeout,
    content;

function getSymbol({ key, shift }) {
    const ALL_KEYS = {
        Numpad0: ['0', '0'], Numpad1: ['1', '1'], Numpad2: ['2', '2'], Numpad3: ['3', '3'], Numpad4: ['4', '4'],
        Numpad5: ['5', '5'], Numpad6: ['6', '6'], Numpad7: ['7', '7'], Numpad8: ['8', '8'], Numpad9: ['9', '9'],
        Numrow0: ['0', ')'], Numrow1: ['1', '!'], Numrow2: ['2', '@'], Numrow3: ['3', '#'], Numrow4: ['4', '$'],
        Numrow5: ['5', '%'], Numrow6: ['6', '^'], Numrow7: ['7', '&'], Numrow8: ['8', '*'], Numrow9: ['9', '('],
        Backslash: ['\\', '|'], Slash: ['/', '?'], Comma: [',', '<'], Period: ['.', '>'], Minus: ['-', '_'],
        Quote: ['\'', '\"'], Semicolon: [';', ':'], LBracket: ['[', '{'], RBracket: [']', '}'], Equal: ['=', '+'],
        Backquote: ['`', '~'], Enter: ['↴', '↴'], Backspace: ['⌫', '⌫'], Space: ['⎵', '⎵'], Delete: ['<Del>', '<Del>'],
        Tab: ['⇥', '⇥'], Right: ['⤑', '⤑'], Left: ['⬸', '⬸'], Up: ['⇡', '⇡'], Down: ['⇣', '⇣'],
    }

    if(ALL_KEYS[key] !== undefined) {
        return ALL_KEYS[key][shift ? 1 : 0];
    }
}

function getKey({ key, ctrl, shift }) {
    if(key.length === 1) {
        key = shift ? key.toUpperCase() : key.toLowerCase();
    } else {
        key = getSymbol({ key, shift });
    }

    if(key === undefined)
        return;

    return ctrl ? `⇧${key}` : key;
}

function updateDisplay() {
    content.innerText = keysQueue.join('');
}

function pushKeyPress(key) {
    if(key === undefined)
        return;

    content.style.opacity = '100%';

    keysQueue.push(key);

    if(keysQueue.length >= 24)
        keysQueue.shift();

    clearTimeout(keysTimeout);
    clearTimeout(transitionTimeout);

    updateDisplay();

    keysTimeout = setTimeout(() => {
        transitionTimeout = setTimeout(() => {
            keysQueue = [];
            updateDisplay();
            content.style.transition = '';
        }, 400);

        keysTimeout = undefined;
        requestAnimationFrame(() => {
            content.style.transition = '';
            content.style.opacity = '100%';
            requestAnimationFrame(() => {
                content.style.transition = 'opacity 0.4s';
                content.style.opacity = '0%';
            })
        })
    }, 2500);
}

socket.addEventListener('open', () => {
    content = document.getElementById("display");
    content.style.opacity = '0%';
    console.log('WebSocket connection established');
});


socket.addEventListener('message', async (event) => {
    const data = JSON.parse(await event.data.text());
    switch(data.event) {
        case 'KeyPress': {
            if(!data.key.endsWith('Key'))
                return;

            data.key = data.key.slice(0, -3);
            pushKeyPress(getKey(data));
            break;
        }
    }
});

socket.addEventListener('close', () => {
    document.getElementById("display").innerText = 'Websocket not connected';
});
