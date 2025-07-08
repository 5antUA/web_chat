// LOGIN ELEMENTS
let nicknameField = document.getElementById("nickname");
let ligonBtn = document.getElementById("loginBtn");

// CHAT ELEMENTS
let sendBtn = document.getElementById("sendBtn");
let messageField = document.getElementById("messageInput");
let textArea = document.getElementById("chatOutput");

//LOGIC

sendBtn.addEventListener("click", function () {
    let message = messageField.value;

    if (isNullOrEmpty(message)) {
        return;
    }

    messageField.value = "";
    textArea.innerHTML += `${message}<br>`;
});

function isNullOrEmpty(string) {
    return string == null || string.length <= 0;
}
