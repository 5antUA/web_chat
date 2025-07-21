import { concatPath } from "../utils/paths.js";
import { jwt_parse } from "../utils/jwt_parse.js";

let nicknameText = document.getElementById("nickname");
let chatTextArea = document.getElementById("chat-window");

let messageInput = document.getElementById("message-input");
let sendButton = document.getElementById("send-btn");

let profileButton = document.getElementById("profile-btn");
let settingsButton = document.getElementById("settings-btn");
let logoutButton = document.getElementById("logout-btn");

const jwt = localStorage.getItem("jwt");
const jwt_payload = jwt_parse(jwt);

loadPage();

function loadPage() {
    fetch(concatPath("/profiles/some"), {
        method: "GET",
        headers: {
            Authorization: `Bearer ${jwt}`,
        },
    });

    nicknameText.textContent = `${jwt_payload.username} (${jwt_payload.role_name})`;
}
