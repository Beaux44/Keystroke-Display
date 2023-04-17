const socket = new WebSocket(address);
let keysQueue = [],
    keysTimeout,
    transitionTimeout,
    content,
    combo = 1,
    combo_streak = false;

const ALL_KEYS = {
    Numpad0: ['0', '0'], Numpad1: ['1', '1'], Numpad2: ['2', '2'], Numpad3: ['3', '3'], Numpad4: ['4', '4'],
    Numpad5: ['5', '5'], Numpad6: ['6', '6'], Numpad7: ['7', '7'], Numpad8: ['8', '8'], Numpad9: ['9', '9'],
    Numrow0: ['0', ')'], Numrow1: ['1', '!'], Numrow2: ['2', '@'], Numrow3: ['3', '#'], Numrow4: ['4', '$'],
    Numrow5: ['5', '%'], Numrow6: ['6', '^'], Numrow7: ['7', '&'], Numrow8: ['8', '*'], Numrow9: ['9', '('],
    Backslash: ['\\', '|'], Slash: ['/', '?'], Comma: [',', '<'], Period: ['.', '>'], Minus: ['-', '_'],
    Quote: ['\'', '"'], Semicolon: [';', ':'], LBracket: ['[', '{'], RBracket: [']', '}'], Equal: ['=', '+'],
    Backquote: ['`', '~'], Enter: ['↴', '↴'], Backspace: ['⌫', '⌫'], Space: ['⎵', '⎵'], Delete: ['⌦', '⌦'],
    Tab: ['⇥', '⇥'], Right: ['⤑', '⤑'], Left: ['⬸', '⬸'], Up: ['⇡', '⇡'], Down: ['⇣', '⇣'],
};

function getKey({ key, ctrl, shift }) {
    if(key.length === 1) {
        key = shift ? key.toUpperCase() : key.toLowerCase();
    } else if(ALL_KEYS[key] !== undefined) {
        key = ALL_KEYS[key][shift ? 1 : 0];
    } else {
        key = undefined;
    }

    if(key === undefined)
        return;

    return ctrl ? `⇧${key}` : key;
}

function updateDisplay() {
    content.innerHTML = '';
    keysQueue.forEach(element => {
        // Handles combos
        if (Array.isArray(element)) {
            let k = document.createElement('span');
            k.innerText = `${element[0]}`;
            let cmb = document.createElement('span');
            cmb.innerText = `x${element[1]}`;
            cmb.style.fontSize = '25px';
            content.appendChild(k);
            content.appendChild(cmb);
        // Regular keys
        } else {
            let el = document.createElement('span');
            el.innerText = element;
            content.appendChild(el);
        }
    });
}

function pushKeyPress(key) {
    if(key === undefined)
        return;

    content.style.opacity = '100%';
    if (key === keysQueue[keysQueue.length - combo] && !(combo_streak)) {
        switch(combo) {
            case 1:
                // Handle first combo
                keysQueue.push(key);
                combo = combo + 1;
                break;
            case 2:
                // Remove last 2 items, so we can have a 'combo key' 
                // Set combo_streak to true so we can handle the followingk keystokes differently
                keysQueue.pop();
                keysQueue.pop();
                combo = combo + 1;
                keysQueue.push([key, combo]);
                combo_streak = true;
                break;
        }


    } else if (combo_streak){
        // check if the next key is another combo
        if (key === keysQueue[keysQueue.length -1][0]) {
            // if it is just edit the second value of the array in the last position of keysque
            combo = combo + 1;
            keysQueue[keysQueue.length -1][1] = combo;
        } else {
            keysQueue.push(key);
            combo = 1;
            combo_streak = false;
        }
    } else {
        keysQueue.push(key);
        combo = 1;
        combo_streak = false;
    }


    if(keysQueue.length >= 24)
        keysQueue.shift();


    clearTimeout(keysTimeout);
    clearTimeout(transitionTimeout);

    updateDisplay();

    keysTimeout = setTimeout(() => {
        transitionTimeout = setTimeout(() => {
            keysQueue = [];
            combo_streak = false;
            combo = 1;
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
            });
        });
    }, 2500);
}

socket.addEventListener('open', () => {
    content = document.getElementById('display');
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
    document.getElementById('display').innerText = 'Websocket not connected';
});
