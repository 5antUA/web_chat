import { concatPath } from "../utils/paths.js";
import { checkSession } from "../root/check_session.js";

let nicknameText = document.getElementById("nickname");
let chatTextArea = document.getElementById("chat-window");

let messageInput = document.getElementById("message-input");
let sendButton = document.getElementById("send-btn");

let profileButton = document.getElementById("profile-btn");
let settingsButton = document.getElementById("settings-btn");
let logoutButton = document.getElementById("logout-btn");

const jwt = localStorage.getItem("jwt");

loadPage();

function loadPage() {
    // checkSession();

    const jwtPayload = jwtLoader(jwt);

    nicknameText.textContent = `${jwtPayload.username} (${jwtPayload.role_name})`;
}
