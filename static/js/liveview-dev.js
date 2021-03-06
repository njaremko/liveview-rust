import morphdom from "morphdom";
const $ = require('jquery');

window.morphdom = morphdom;
window.$ = $;

let conn = null;

let content = document.getElementById('content');

function connect() {
    disconnect();
    let wsUri = (window.location.protocol === 'https:' && 'wss://' || 'ws://') + window.location.host + '/ws/';
    conn = new WebSocket(wsUri);
    console.log('Connecting...');
    conn.onopen = function () {
        console.log('Connected.');
    };
    conn.onmessage = function (e) {
        let new_content = document.createElement('div');
        new_content.setAttribute('id', 'content');
        new_content.innerHTML = e.data;
        morphdom(content, new_content, {
            onBeforeElUpdated: function (fromEl, toEl) {
                if (toEl.tagName === 'INPUT') {
                    toEl.value = fromEl.value;
                }
            },

        });
        // attach();
    };
    conn.onclose = function () {
        console.log('Disconnected.');
        conn = null;
    };
}

function disconnect() {
    if (conn != null) {
        log('Disconnecting...');
        conn.close();
        conn = null;
    }
}

function send_event(kind, event, data = null) {
    let json = JSON.stringify({
        "kind": kind,
        "event": event,
        "data": data,
    });
    console.log(json);
    conn.send(json);
}

const CLICK_EVENT = 'click';
const SUBMIT_EVENT = 'submit';
const INPUT_EVENT = 'input';

function attach() {
    let clickElems = document.querySelectorAll('[rust-click]');
    for (let i = 0; i < clickElems.length; i++) {
        clickElems[i].addEventListener(CLICK_EVENT, function (e) {
            e.preventDefault();
            let val = clickElems[i].getAttribute('rust-click');
            send_event(CLICK_EVENT, val);
        });
    }

    let submitElems = document.querySelectorAll('[rust-submit]');
    for (let i = 0; i < submitElems.length; i++) {
        submitElems[i].addEventListener(SUBMIT_EVENT, function (e) {
            e.preventDefault();
            let form = $(this).serialize();
            let event = submitElems[i].getAttribute('rust-submit');
            send_event(SUBMIT_EVENT, event, form);
        });
    }

    let keydownElems = document.querySelectorAll('[rust-keydown]');
    for (let i = 0; i < keydownElems.length; i++) {
        keydownElems[i].addEventListener(INPUT_EVENT, function (e) {
            let event = keydownElems[i].getAttribute('rust-keydown');
            let val = $(this).val();
            send_event(INPUT_EVENT, event, val);
        });
    }
}

connect();
attach();