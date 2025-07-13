import { concatPath } from "./paths.js";

// LOGIN ELEMENTS
let userField = document.getElementById("nickname");
let loginBtn = document.getElementById("loginBtn");
let userInfo = document.getElementById("userDisplay");

// CHAT ELEMENTS
let sendBtn = document.getElementById("sendBtn");
let msgField = document.getElementById("messageInput");
let chatArea = document.getElementById("chatOutput");

//LOGIC
sendBtn.addEventListener("click", function () {
    let message = msgField.value;

    if (isNullOrEmpty(message)) {
        return;
    }

    msgField.value = "";
    chatArea.innerHTML += `${message}<br>`;
});

loginBtn.addEventListener("click", function () {
    let name = userField.value;

    if (isNullOrEmpty(name)) {
        return;
    }

    fetch(concatPath(`/users/${name}`))
        .then((data) => data.json())
        .then((data) => (userInfo.textContent = `Вітаємо, ${data.username}!`))
        .catch(_ => {
            userInfo.textContent = "Вибачте, такого користувача не знайдено.";
        });
});

function isNullOrEmpty(string) {
    return !string || string.length == 0;
}
